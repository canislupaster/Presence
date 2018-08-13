module Model

open System
open System.Diagnostics
open System.IO
open Avalonia.Diagnostics.ViewModels

open DiscordRPC
open Newtonsoft.Json

open System.Threading.Tasks
open System.ComponentModel
open RPC

open System.Text.RegularExpressions

let (|RegexMatch|_|) pattern (groupnum:int) input =
        let m = Regex.Match(input,pattern)
        if (m.Success) then Some m.Groups.[groupnum].Value else None

type DataModel() as x =
    inherit ViewModelBase ()

    let cecEvent = new Event<PropertyChangedEventHandler,PropertyChangedEventArgs>()

    interface INotifyPropertyChanged with
        [<CLIEvent>]
        member x.PropertyChanged = cecEvent.Publish

    member x.PropChanged name =
        cecEvent.Trigger (x, new PropertyChangedEventArgs(name))

    member val Status = "..." with get,set
    member val AppId="465995275563958272" with get,set
    member val UpdateTime = 15 with get,set
    member val Slideshow = true with get,set
    member val ImageNum = 1 with get, set
    member val Image = "doge" with get, set
    member val SmallImage = "smoldoggo" with get, set
    member val MaxImageNum = 8 with get,set
    member val State = "state" with get,set
    member val Details = "details" with get,set

    member public x.SStatus y =
        printfn "Status: %s" y
        x.Status <- y
        x.PropChanged "Status"

    member public x.Reconnect () =
        async {
            x.SStatus "Connecting"
            let! client = RPCMailbox.PostAndAsyncReply (fun i -> Reconnect (x.AppId, i))

            client.OnError.Add (fun err -> err.Message |> sprintf "Error: %s" |> x.SStatus)
            client.OnConnectionFailed.Add (fun err -> err |> sprintf "Connection failed: %A" |> x.SStatus)
            client.OnReady.Add (fun _ -> x.SStatus "Ready")

            client.Initialize () |> ignore
        } |> Async.Start

    member public x.UpdateImage () =
        if x.Slideshow then
            match x.ImageNum with
                | y when y>=x.MaxImageNum ->
                    x.ImageNum <- 1
                | _ -> x.ImageNum <- x.ImageNum+1
        x.PropChanged "ImageNum"

    member public x.Update () =
        x.SStatus "Updating"
        x.UpdateImage ()

        let slide = match x.Slideshow with
                    | true -> Some {N=x.ImageNum; Max=x.MaxImageNum; Time=TimeSpan.FromSeconds (float x.UpdateTime);}
                    | false -> None

        {Slideshow=slide; State=x.State; Details=x.Details; LargeImage=x.Image; SmallImage=x.SmallImage}
            |> Update |> RPCMailbox.Post
        x.SStatus "Ready"

    member public x.RunRPC () =
        async {
            x.Reconnect ()

            let rec loop (x:DataModel) = async {
                x.Update ()

                do! Async.Sleep (x.UpdateTime*1000)
                return! loop x
            }

            do loop x |> Async.Start
        } |> Async.Start

    member public x.ShowHelp () =
        let url = "https://thomas-qm.github.io/RichPresenceClient/"
        Process.Start(new ProcessStartInfo("cmd", sprintf "/c start %s" (url.Replace("&", "^&") )))

let mutable mainmodel =
    if File.Exists("config.json") then
        File.ReadAllText ("config.json") |> JsonConvert.DeserializeObject<DataModel>
    else
        DataModel()