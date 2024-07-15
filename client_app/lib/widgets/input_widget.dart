import 'dart:ui';

import 'package:flutter/material.dart';

class InputTextFieldWidget extends StatelessWidget {
  final TextEditingController textEditingController;
  final String hintText;

  const InputTextFieldWidget(this.textEditingController, this.hintText,
      {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 46,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(23), // Bordures arrondies
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            Colors.white.withOpacity(0.5),
            Colors.white24
          ], // Effet miroir
        ),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 10,
            offset: const Offset(0, 5), // Ombre pour un effet de profondeur
          ),
        ],
      ),
      child: ClipRRect(
        borderRadius:
            BorderRadius.circular(23), // Bordures arrondies pour le TextField
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 2, sigmaY: 2), // Effet de flou
          child: TextField(
            controller: textEditingController,
            decoration: InputDecoration(
              border: InputBorder.none, // Suppression de la bordure
              fillColor: Colors.white.withOpacity(0.5),
              filled: true,
              hintText: hintText,
              hintStyle: TextStyle(color: Colors.grey[700]),
              contentPadding:
                  const EdgeInsets.only(left: 20, bottom: 15, right: 20),
            ),
          ),
        ),
      ),
    );
  }
}
