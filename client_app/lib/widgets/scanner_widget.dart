import 'package:flutter/material.dart';

class ScannerWidget extends StatelessWidget {
  final String title;
  final String subTitle;
  final double sizeTitle;
  final Color colorScanner;
  final VoidCallback onCircleTap;
  final ValueNotifier<bool> isPressed;

  const ScannerWidget({
    Key? key,
    required this.title,
    required this.subTitle,
    required this.sizeTitle,
    required this.colorScanner,
    required this.onCircleTap,
    required this.isPressed,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: <Widget>[
        Padding(
            padding: const EdgeInsets.only(top: 0.0, bottom: 30.0),
            child: Text(
              title,
              textAlign: TextAlign.center,
              style: TextStyle(
                  color: const Color(0xFF4B4B4B), fontSize: sizeTitle),
            )),
        ValueListenableBuilder<bool>(
          valueListenable: isPressed, // Écouter les changements de isPressed
          builder: (context, isPressedValue, child) {
            // Rebuilder le widget en fonction de isPressedValue
            return Center(
              child: Theme(
                  data: Theme.of(context).copyWith(
                    splashColor: Colors.transparent,
                    highlightColor: Colors.transparent,
                    hoverColor: Colors.transparent,
                  ),
                  child: InkWell(
                    onTap: onCircleTap,
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 300),
                      curve: Curves.easeInOut,
                      width: 200,
                      height: 200,
                      decoration: BoxDecoration(
                          color: isPressedValue
                              ? colorScanner
                              : Colors.transparent,
                          shape: BoxShape.circle,
                          border: Border.all(color: colorScanner, width: 13)),
                      child: Center(
                        child: Text(
                          subTitle,
                          style: TextStyle(
                              color: isPressedValue
                                  ? Colors.white
                                  : const Color(0xFF4B4B4B),
                              fontSize: 28),
                        ),
                      ),
                    ),
                  )),
            );
          },
        ),
      ],
    );
  }
}
