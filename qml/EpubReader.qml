import QtQuick 2.9
import QtQuick.Controls 2.5
import QtWebView 1.1
import QtQuick.Layouts 1.3
ColumnLayout{
	property bool showAddButton: false
	property bool showBackButton: true

    property string readerUrl
	property string bookUrl
	property string epubCfi

	property string bookUUID;

	property string title
	function backButtonPressed(){
		epubWebView.runJavaScript("get_cfi();", function(epubCfi){
					LibraryModel.setBookLocation(epubCfi, row);
				});
		stackView.pop();
	}
	function setBookUrl(bookUrl, epubCfi){
		epubWebView.url = "file:///home/johan/.local/share/eBiblis/web/epubReader.html?bookUrl=" + bookUrl + "&epubCfi=" + epubCfi
		//var functionCall = "setBook(\"" + bookUrl +"\", \""+ epubCfi +"\");";
		//console.log(functionCall)
		//epubWebView.runJavaScript(functionCall);
	}

	WebView{
		function nextPage(){
			epubWebView.runJavaScript("nextPage();", function(location){
						LibraryModel.setBookLocation(location, row);
						console.log("Next page to: " + location);
					});
		}
		function prevPage(){
			epubWebView.runJavaScript("prevPage()", function(location){
						LibraryModel.setBookLocation(location, row);
						console.log("Next page to: " + location);
					});
		}


		id:epubWebView
		anchors.fill: parent
		//url: readerUrl + "?bookUrl=" + bookUrl + "&epubCfi=" + epubCfi

		Keys.onRightPressed: epubWebView.runJavaScript("nextPage();", function(location){
			LibraryModel.setBookLocation(bookUUID, location);
		});
		Keys.onLeftPressed: epubWebView.runJavaScript("prevPage()", function(location){
			LibraryModel.setBookLocation(bookUUID, location);
		});

		}
}

