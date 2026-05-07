import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app/router.dart';
import 'app/theme.dart';

void main() {
  runApp(const ProviderScope(child: WaveSightApp()));
}

class WaveSightApp extends ConsumerWidget {
  const WaveSightApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);
    return MaterialApp.router(
      title: 'WaveSight',
      theme: buildAppTheme(brightness: Brightness.dark),
      routerConfig: router,
      debugShowCheckedModeBanner: false,
    );
  }
}
