import 'package:client_app/main.dart';
import 'package:client_app/provider/user_provider.dart';
import 'package:client_app/widgets/custom_app_bar.dart';
import 'package:client_app/widgets/custom_button_widget.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:provider/provider.dart';

class ProfileScreen extends StatelessWidget {
  const ProfileScreen({super.key});

  Future<void> _logout(BuildContext context) async {
    const storage = FlutterSecureStorage();
    await storage.delete(key: "keycloak_token");
    final userProvider = Provider.of<UserProvider>(context, listen: false);
    userProvider.clearUserInfo();
    keycloakWrapper.logout();
  }

  @override
  Widget build(BuildContext context) {
    final userProvider = Provider.of<UserProvider>(context);
    final user = userProvider.user ?? {};
    logger.i("user PS $user");

    // Traiter le cas où email_verified est null
    final emailVerified = user['email_verified'] ?? false;

    return Scaffold(
      appBar: const CustomAppBar(title: 'Paramètres'),
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const SizedBox(height: 20),
              const Center(
                child: Text(
                  'Vos informations',
                  style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                ),
              ),
              const SizedBox(height: 32),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Nom:',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                  Text(user['name'] ?? 'N/A',
                      style: Theme.of(context).textTheme.bodyLarge),
                ],
              ),
              const SizedBox(height: 16),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Prénom:',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                  Text(user['given_name'] ?? 'N/A',
                      style: Theme.of(context).textTheme.bodyLarge),
                ],
              ),
              const SizedBox(height: 16),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Nom de famille:',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                  Text(user['family_name'] ?? 'N/A',
                      style: Theme.of(context).textTheme.bodyLarge),
                ],
              ),
              const SizedBox(height: 16),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Nom d\'utilisateur:',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                  Text(user['preferred_username'] ?? 'N/A',
                      style: Theme.of(context).textTheme.bodyLarge),
                ],
              ),
              const SizedBox(height: 16),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text('Email:',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                  Row(
                    children: [
                      Text(user['email'] ?? 'N/A',
                          style: Theme.of(context).textTheme.bodyLarge),
                      const SizedBox(width: 8),
                      Icon(
                        emailVerified ? Icons.check_circle : Icons.cancel,
                        color: emailVerified ? Colors.green : Colors.red,
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 32),
              Center(
                child: CustomButton(
                  onPressed: () => _logout(context),
                  title: 'Se déconnecter',
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
