
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:nekalkulacka/src/ui/pages/main_page.dart';



class NekalkulatorApp extends StatelessWidget {
  const NekalkulatorApp({
    Key? key,
  }) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {


    return MultiBlocProvider(
      providers: const [
      
      ],
      child: MaterialApp(
          debugShowCheckedModeBanner: false,
          title: "Nekalkulator",
          theme: ThemeData(
              scaffoldBackgroundColor: Colors.white,
              canvasColor: Colors.amber,
              textTheme: GoogleFonts.pollerOneTextTheme()),
          home:const MainPage(),
    ));
  }
}
