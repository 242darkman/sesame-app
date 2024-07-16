import 'dart:io' show Platform;

String getBundleIdentifier() {
  // Identifiant par défaut (peut-être utilisé pour les plateformes non spécifiées)
  String bundleIdentifier = 'default.identifier';

  if (Platform.isIOS) {
    bundleIdentifier = 'io.flutter.flutter.app';
  } else if (Platform.isAndroid) {
    bundleIdentifier = 'com.client.app.android';
  }

  return bundleIdentifier;
}
