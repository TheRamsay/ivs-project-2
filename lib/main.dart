import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:flutter/material.dart';
import 'app.dart';
import 'package:firebase_core/firebase_core.dart';
//import 'firebase_options.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

// await Firebase.initializeApp(
//   options: DefaultFirebaseOptions.currentPlatform,
// );
// FirebaseAnalytics analytics = FirebaseAnalytics.instance;
  runApp(const NekalkulatorApp());
}
