
module RPC

open System

open DiscordRPC
open DiscordRPC.Logging

type AppId = string
type Update = {Num:int; MaxNum:int option; Time:TimeSpan option; State:string; Details:string}
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

            | Update {Num=i; MaxNum=m; Time=t; State=s; Details=d}, Some c ->
                c.Invoke () |> ignore

                let presence = RichPresence()
                presence.State <- s
                presence.Details <- d

                presence.Assets <- Assets ()
                presence.Assets.LargeImageKey <- i |> string

                match m with | Some x ->
                                presence.Party <- Party ()
                                presence.Party.ID <- Secrets.CreateFriendlySecret(new Random())
                                presence.Party.Size <- i
                                presence.Party.Max <- x | None -> ()

                match t with
                    | Some t ->
                        let time = Timestamps()
                        time.Start <- DateTime.UtcNow |> Nullable
                        time.End <- DateTime.UtcNow + t |> Nullable
                        presence.Timestamps <- time
                    | None -> ()

                c.SetPresence presence
                client

            | _ -> client

    return! RPCLoop newstate x
}


let RPCMailbox = MailboxProcessor.Start (RPCLoop None)