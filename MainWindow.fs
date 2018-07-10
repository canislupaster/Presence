namespace DiscordRPCClient

open Avalonia
open Avalonia.Controls
open Avalonia.Markup.Xaml

open System.Reflection

type MainWindow () as this =
    inherit Window()

    #if DEBUG
    do DevToolsExtensions.AttachDevTools (this)
    #endif
    do this.InitializeComponent()

    member this.InitializeComponent() =
        AvaloniaXamlLoader.Load (this)