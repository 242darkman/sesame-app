import 'package:client_app/main.dart';
import 'package:client_app/utils/env.dart';
import 'package:client_app/utils/get_bundle_identifier.dart';
import 'package:client_app/widgets/custom_button_widget.dart';
import 'package:flutter/material.dart';
import 'package:keycloak_wrapper/keycloak_wrapper.dart';

class LoginScreen extends StatelessWidget {
  const LoginScreen({super.key});

  // Login using the given configuration.
  Future<bool> login() async {
    final bundleIdentifier = getBundleIdentifier();
    final config = KeycloakConfig(
      bundleIdentifier: bundleIdentifier,
      clientId: Environment.KEYCLOAK_CLIENT_ID,
      frontendUrl: Environment.KEYCLOAK_URL,
      realm: Environment.KEYCLOAK_REALM,
    );

    // Check if user has successfully logged in.
    final isLoggedIn = await keycloakWrapper.login(config);

    return isLoggedIn;
  }

  @override
  Widget build(BuildContext context) => Scaffold(
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
              // Centre le bouton horizontalement et verticalement
              child: CustomButton(
                onPressed: login,
                title: 'Se connecter',
              ),
            ),
          ],
        ),
      );
}
