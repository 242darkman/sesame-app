import 'package:client_app/main.dart';
import 'package:client_app/provider/user_provider.dart';
import 'package:client_app/utils/env.dart';
import 'package:client_app/utils/get_bundle_identifier.dart';
import 'package:client_app/widgets/custom_button_widget.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:keycloak_wrapper/keycloak_wrapper.dart';
import 'package:provider/provider.dart';

class LoginScreen extends StatelessWidget {
  const LoginScreen({super.key});

  // Login using the given configuration.
  Future<bool> login(BuildContext context) async {
    final bundleIdentifier = getBundleIdentifier();
    final config = KeycloakConfig(
      bundleIdentifier: bundleIdentifier,
      clientId: Environment.KEYCLOAK_CLIENT_ID,
      frontendUrl: Environment.KEYCLOAK_URL,
      realm: Environment.KEYCLOAK_REALM,
    );

    // Check if user has successfully logged in.
    final isLoggedIn = await keycloakWrapper.login(config);
    if (isLoggedIn) {
      logger.i("Utilisateur connecté avec succès.");
      logger.i("Access token: ${keycloakWrapper.accessToken}");
      const storage = FlutterSecureStorage();
      await storage.write(
          key: "keycloak_token", value: keycloakWrapper.accessToken);

      // Stockez les informations de l'utilisateur dans le UserProvider
      final userProvider = Provider.of<UserProvider>(context, listen: false);
      userProvider.setUserInfo(
        accessToken: keycloakWrapper.accessToken!,
        user: await keycloakWrapper.getUserInfo() ?? {},
      );
    }

    return isLoggedIn;
  }

  @override
  Widget build(BuildContext context) => Scaffold(
        backgroundColor: Colors.white,
        body: Stack(
          fit: StackFit.expand,
          children: [
            Image.asset(
              'assets/images/image_accueil.png',
              fit: BoxFit.cover,
            ),
            Align(
              alignment: Alignment.topCenter,
              child: Padding(
                padding: const EdgeInsets.only(
                    top:
                        kToolbarHeight), // Utilisez kToolbarHeight pour un positionnement proche de l'app_bar
                child: Image.asset(
                  'assets/images/logo_sesame.jpg',
                  width: 150,
                  height: 150,
                ),
              ),
            ),
            Center(
              child: Padding(
                padding: const EdgeInsets.only(
                    bottom:
                        100.0), // Adjust the bottom padding to move the button up
                child: CustomButton(
                  onPressed: () => login(context),
                  title: 'Se connecter',
                ),
              ),
            ),
          ],
        ),
      );
}
