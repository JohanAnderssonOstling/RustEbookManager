#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QtWebView/QtWebView>
#include <QQmlContext>
#include <QLocale>
#include <QTranslator>
#include "homemodel.h"
#include "librarymodel.h"
int main(int argc, char *argv[])
{
   // qputenv("QT_IM_MODULE", QByteArray("qtvirtualkeyboard"));

    QCoreApplication::setAttribute(Qt::AA_ShareOpenGLContexts);
    QGuiApplication app(argc, argv);
    
    	QtWebView::initialize();

    QTranslator translator;
    const QStringList uiLanguages = QLocale::system().uiLanguages();
    for (const QString &locale : uiLanguages) {
        const QString baseName = "EbookTest_" + QLocale(locale).name();
        if (translator.load(":/i18n/" + baseName)) {
            app.installTranslator(&translator);
            break;
        }
    }

    QQmlApplicationEngine engine;
    const QUrl url("../qml/main.qml");
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);

    HomeModel homeModel(0, &engine);
	homeModel.updateLibraryList();
    engine.rootContext()->setContextProperty("HomeModel", &homeModel);
	qmlRegisterType<LibraryModel>("johandost.LibraryModel", 1, 0, "LibraryModel");
    engine.load(url);
    return app.exec();
}
