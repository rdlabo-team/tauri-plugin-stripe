import Tauri

@_cdecl("init_plugin_stripe_identity")
func initPlugin() -> Plugin {
  return StripeIdentityPlugin()
}
