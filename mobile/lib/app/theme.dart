import 'package:flutter/material.dart';

ThemeData buildAppTheme({required Brightness brightness}) {
  final isDark = brightness == Brightness.dark;
  final base = ThemeData(brightness: brightness, useMaterial3: true);
  return base.copyWith(
    scaffoldBackgroundColor: isDark ? const Color(0xFF0B0D10) : Colors.white,
    colorScheme: base.colorScheme.copyWith(
      primary: const Color(0xFF5EEAD4),
      secondary: const Color(0xFFF5C542),
      error: const Color(0xFFEF4444),
    ),
    cardTheme: CardTheme(
      color: isDark ? const Color(0xFF13161C) : Colors.white,
      elevation: 0,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
    ),
    appBarTheme: AppBarTheme(
      backgroundColor: isDark ? const Color(0xFF0B0D10) : Colors.white,
      foregroundColor: isDark ? Colors.white : Colors.black,
      elevation: 0,
    ),
  );
}
