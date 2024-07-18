import 'package:flutter/material.dart';

Color getTypeColor(String type) {
  switch (type) {
    case "Propreté":
      return const Color(0xFF51BEE0);
    case "Savon manquant":
      return const Color(0xFF5170E0);
    default:
      return const Color(0xFF30959B);
  }
}
