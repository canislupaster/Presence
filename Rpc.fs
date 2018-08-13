
module RPC

open System

open DiscordRPC
open DiscordRPC.Logging

type AppId = string
type Slideshow = {N:int; Max:int; Time:TimeSpan}
type Update = {SmallImage:string; LargeImage:string; Slideshow:Slideshow option; State:string; Details:string}
type RpcMsg =
    | Reconnect of AppId*AsyncReplyChannel<DiscordRpcClient>
    | Update of Update

let rec RPCLoop (client:DiscordRpcClient option) (x:MailboxProcessor<RpcMsg>) = async {
    let! msg = x.Receive ()
    let newstate =
        match msg, client with
            | Reconnect (x, reply), y ->
                match y with | Some y -> y.Dispose () | None -> ()
                let newc = new DiscordRpcClient (x, false)
                reply.Reply newc

                newc |> Some

            | Update {SmallImage=small; LargeImage=large; Slideshow=slide; State=s; Details=d}, Some c ->
                c.Invoke () |> ignore

                let presence = RichPresence()
                presence.State <- s
                presence.Details <- d

                presence.Assets <- Assets ()
                presence.Assets.SmallImageKey <- small

                match slide with 
                    | Some {N=num; Max=max; Time=t;} ->
                        presence.Party <- Party ()
                        presence.Party.ID <- Secrets.CreateFriendlySecret(new Random())
                        presence.Party.Size <- num
                        presence.Assets.LargeImageKey <- num |> sprintf "%s-%i" large
                        presence.Party.Max <- max

                        let time = Timestamps()
                        time.Start <- DateTime.UtcNow |> Nullable
                        time.End <- DateTime.UtcNow + t |> Nullable
                        presence.Timestamps <- time
                    | None -> presence.Assets.LargeImageKey <- large

                c.SetPresence presence
                client

            | _ -> client

    return! RPCLoop newstate x
}


let RPCMailbox = MailboxProcessor.Start (RPCLoop None)