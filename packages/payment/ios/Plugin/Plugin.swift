import Tauri

@_cdecl("init_plugin_stripe_payment")
func initPlugin() -> Plugin {
  return StripePlugin()
}
