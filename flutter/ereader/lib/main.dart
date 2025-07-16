import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:epubx/epubx.dart';
import 'package:flutter/services.dart';
import 'package:file_picker/file_picker.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:flutter/foundation.dart' show kIsWeb;

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'EPUB Marquee Reader',
      home: EpubStreamingReader(),
    );
  }
}

class EpubStreamingReader extends StatefulWidget {
  @override
  State<EpubStreamingReader> createState() => _EpubStreamingReaderState();
}

class _EpubStreamingReaderState extends State<EpubStreamingReader> {
  List<EpubChapter>? _chapters;
  bool _loading = false;

  Future<void> pickEpubFile() async {
    setState(() => _loading = true);

    final result = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: ['epub'],
    );

    if (result != null && result.files.single.path != null) {
      Uint8List? bytes;

      if (kIsWeb) {
        bytes = result.files.single.bytes;
      } else {
        bytes = await File(result.files.single.path!).readAsBytes();
      }
      if (bytes != null) {
        final book = await EpubReader.readBook(bytes);
        setState(() {
          _chapters = book.Chapters;
          _loading = false;
        });
      }
    } else {
      setState(() {
        _loading = false;
      });
    }
  }

  String _extractText(EpubChapter chapter) {
    final raw = chapter.HtmlContent ?? '';
    final text = raw.replaceAll(RegExp(r'<[^>]*>'), '');
    final children = chapter.SubChapters?.map(_extractText).join(' ') ?? '';
    return '$text $children';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("EPUB Lazy Scroll Reader"),
        actions: [
          IconButton(icon: Icon(Icons.folder_open), onPressed: pickEpubFile),
        ],
      ),
      body:
          _loading
              ? Center(child: CircularProgressIndicator())
              : _chapters == null
              ? Center(child: Text("Select an EPUB file"))
              : ListView.builder(
                scrollDirection: Axis.horizontal,
                itemCount: _chapters!.length,
                itemBuilder: (context, index) {
                  final chapter = _chapters![index];
                  final text = _extractText(
                    chapter,
                  ).replaceAll(RegExp(r'\s+'), ' ');

                  return Container(
                    padding: EdgeInsets.all(16),
                    constraints: BoxConstraints(
                      maxWidth: MediaQuery.of(context).size.width * 2,
                    ),
                    child: Text(
                      text,
                      style: TextStyle(fontSize: 18),
                      softWrap: false,
                    ),
                  );
                },
              ),
    );
  }
}
