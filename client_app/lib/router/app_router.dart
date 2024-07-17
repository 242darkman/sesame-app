import 'package:client_app/screens/door/door_screen.dart';
import 'package:client_app/screens/login/login_screen.dart';
import 'package:client_app/screens/scanner/describe_problem_screen.dart';
import 'package:client_app/screens/scanner/describe_without_scan_screen.dart';
import 'package:client_app/screens/scanner/report_comment_screen.dart';
import 'package:client_app/screens/scanner/report_details_screen.dart';
import 'package:client_app/screens/scanner/reports_screen.dart';
import 'package:client_app/screens/scanner/scanner_screen.dart';
import 'package:client_app/screens/user/profile_screen.dart';
import 'package:go_router/go_router.dart';

GoRouter createRouter(bool isAuthenticated) {
  return GoRouter(
    redirect: (context, state) {
      final isLoginRoute = state.matchedLocation == '/';
      if (isAuthenticated && isLoginRoute) {
        return '/app/scanner';
      } else if (!isAuthenticated && !isLoginRoute) {
        return '/';
      }
      return null;
    },
    routes: [
      GoRoute(
        path: '/',
        builder: (context, state) => const LoginScreen(),
      ),
      GoRoute(
        path: '/app/scanner',
        builder: (context, state) => const ScannerScreen(),
      ),
      GoRoute(
        path: '/app/door',
        builder: (context, state) => const DoorScreen(),
      ),
      GoRoute(
        path: '/app/scanner/report_problem/describe',
        builder: (context, state) => const DescribeProblemScreen(),
      ),
      GoRoute(
        path: '/app/scanner/report_problem/describe_without_scan',
        builder: (context, state) => const DescribeWithoutScanScreen(),
      ),
      GoRoute(
        path: '/app/scanner/reports',
        builder: (context, state) => const ReportsScreen(),
      ),
      GoRoute(
        path: '/app/scanner/report_details',
        builder: (context, state) => const ReportDetailsScreen(),
      ),
      GoRoute(
        path: '/app/scanner/report_comment',
        builder: (context, state) => const ReportCommentScreen(),
      ),
      GoRoute(
        path: '/app/scanner/settings',
        builder: (context, state) => const ProfileScreen(),
      ),
    ],
  );
}
