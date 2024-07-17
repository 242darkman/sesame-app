import 'package:client_app/provider/user_provider.dart';
import 'package:client_app/router/app_router.dart';
import 'package:client_app/services/websocket_service.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:jwt_decoder/jwt_decoder.dart';
import 'package:keycloak_wrapper/keycloak_wrapper.dart';
import 'package:logger/logger.dart';
import 'package:provider/provider.dart';

final keycloakWrapper = KeycloakWrapper();
final scaffoldMessengerKey = GlobalKey<ScaffoldMessengerState>();
final logger = Logger();

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await keycloakWrapper.initialize();
  keycloakWrapper.onError = (e, s) {
    final errorMessage = e.toString();
    if (errorMessage.contains("User cancelled login")) {
      logger.i("Login annulé par l'utilisateur.");
    } else if (errorMessage.contains("org.openid.appauth.general error -3")) {
      logger.w("Authorization failed with error -3: $e");
    } else {
      logger.e(e);
      scaffoldMessengerKey.currentState
        ?..hideCurrentSnackBar()
        ..showSnackBar(SnackBar(content: Text('$e')));
    }
  };

  final webSocketService = WebSocketService();

  const storage = FlutterSecureStorage();
  UserProvider userProvider = UserProvider();

  if (keycloakWrapper.accessToken != null) {
    await storage.write(
        key: "keycloak_token", value: keycloakWrapper.accessToken!);
    final user = await keycloakWrapper.getUserInfo();
    userProvider.setUserInfo(
        accessToken: keycloakWrapper.accessToken, user: user);
    final userId = user!['sub'];
    await webSocketService.connect(userId);
  } else {
    final token = await storage.read(key: "keycloak_token");
    if (token != null && !JwtDecoder.isExpired(token)) {
      final user = await keycloakWrapper.getUserInfo();
      userProvider.setUserInfo(accessToken: token, user: user);
      final userId = user!['sub'];
      await webSocketService.connect(userId);
    }
  }

  final token = await storage.read(key: "keycloak_token");
  logger.i("token $token");

  logger.i("user ${await keycloakWrapper.getUserInfo()}");

  /*if (token?.isEmpty || (token != null && JwtDecoder.isExpired(token))) {
    await storage.delete(key: "keycloak_token");
  }*/

  runApp(MyApp(webSocketService: webSocketService, userProvider: userProvider));
}

class MyApp extends StatelessWidget {
  final WebSocketService webSocketService;
  final UserProvider userProvider;

  const MyApp(
      {super.key, required this.webSocketService, required this.userProvider});

  @override
  Widget build(BuildContext context) => MultiProvider(
        providers: [
          ChangeNotifierProvider(create: (_) => userProvider),
          ChangeNotifierProvider(create: (_) => webSocketService),
        ],
        child: StreamBuilder<bool>(
          initialData: false,
          stream: keycloakWrapper.authenticationStream,
          builder: (context, snapshot) {
            final isAuthenticated = snapshot.data ?? false;
            final router = createRouter(isAuthenticated);

            return MaterialApp.router(
              scaffoldMessengerKey: scaffoldMessengerKey,
              debugShowCheckedModeBanner: false,
              routerDelegate: router.routerDelegate,
              routeInformationParser: router.routeInformationParser,
              routeInformationProvider: router.routeInformationProvider,
            );
          },
        ),
      );
}
