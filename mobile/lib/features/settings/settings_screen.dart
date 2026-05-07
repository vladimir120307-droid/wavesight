import 'package:flutter/material.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        children: [
          const ListTile(
            title: Text('Server URL'),
            subtitle: Text('http://wavesight.local:8081'),
            leading: Icon(Icons.cloud_off_outlined),
          ),
          const Divider(height: 0),
          const ListTile(
            title: Text('Notifications'),
            subtitle: Text('Fall alerts only'),
            leading: Icon(Icons.notifications_outlined),
          ),
          const Divider(height: 0),
          ListTile(
            title: const Text('Honest Mode'),
            subtitle: const Text('Show confidence intervals next to every value'),
            leading: const Icon(Icons.shield_outlined),
            trailing: Switch.adaptive(value: true, onChanged: (_) {}),
          ),
          const Divider(height: 0),
          const ListTile(
            title: Text('Language'),
            subtitle: Text('English'),
            leading: Icon(Icons.language_outlined),
          ),
        ],
      ),
    );
  }
}
