import 'package:client_app/screens/home_screen.dart';
import 'package:client_app/screens/login/login_screen.dart';
import 'package:flutter/material.dart';
import 'package:keycloak_wrapper/keycloak_wrapper.dart';
import 'package:logger/logger.dart';

final keycloakWrapper = KeycloakWrapper();
final scaffoldMessengerKey = GlobalKey<ScaffoldMessengerState>();
final logger = Logger();

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Initialize the plugin at the start of your app.
  await keycloakWrapper.initialize();
  // Listen to the errors caught by the plugin.
  keycloakWrapper.onError = (e, s) {
    final errorMessage = e.toString();
    if (errorMessage.contains("User cancelled login")) {
      logger.i("Login annulé par l'utilisateur.");
    } else if (errorMessage.contains("org.openid.appauth.general error -3")) {
      // Ignore this specific error and log it instead of showing a snackbar
      logger.w("Authorization failed with error -3: $e");
    } else {
      logger.e(e);
      scaffoldMessengerKey.currentState
        ?..hideCurrentSnackBar()
        ..showSnackBar(SnackBar(content: Text('$e')));
    }
  };
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
        scaffoldMessengerKey: scaffoldMessengerKey,
        debugShowCheckedModeBanner: false,
        // Listen to the user authentication stream.
        home: StreamBuilder<bool>(
          initialData: false,
          stream: keycloakWrapper.authenticationStream,
          builder: (context, snapshot) =>
              snapshot.data! ? const HomeScreen() : const LoginScreen(),
        ),
      );
}
