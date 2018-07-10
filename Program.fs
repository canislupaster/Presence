open System
open System.IO

open Newtonsoft.Json

open Avalonia

open DiscordRPCClient
open Model
open RPC

[<EntryPoint>]
let main argv =
    //mainmodel.RunRPC ()

    let app =   AppBuilder.Configure<App>()
                    .UsePlatformDetect()
    app.Start<MainWindow>((fun () -> mainmodel :> obj))

    File.WriteAllText ("config.json", mainmodel |> JsonConvert.SerializeObject)

    0
