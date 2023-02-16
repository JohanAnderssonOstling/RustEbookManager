
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia
import QtMultimedia 5.15


Item {
Layout.fillWidth: true
Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
implicitHeight: 100
property var pos: 360000
    MediaPlayer {
        id: audioPlayer
        source: "file:///home/johandost/Hem/Torrent/Bad Blood Secrets and Lies in a Silicon Valley Startup by John Carreyrou (Unabridged)/Bad Blood Secrets and Lies in a Silicon Valley Startup (Unabridged) - 01.mp3"
        audioOutput: AudioOutput {}
        position: pos
    }

    //Audio{
        //id: audioPlayer
        //source: "file:///home/johandost/Hem/Documents/Building a Second Brain/Building a Second Brain_ A Proven Method to Organi.m4b"

    //}

    RowLayout {
        spacing: 10
        Layout.fillWidth: true
        Button{
            text: "Play"
            onPressed:  {
                if (audioPlayer.playbackState == MediaPlayer.PlayingState) {
                    audioPlayer.pause()
                } else {
                    audioPlayer.play()
                }
            }
        }

        Text {
            id: mediaTime
            horizontalAlignment: Text.AlignLeft
            text: {
                var hours = Math.floor(audioPlayer.position / 3600000)
                                var minutes = Math.floor(audioPlayer.position / 60000 - hours * 60)
                                var seconds = Math.floor(audioPlayer.position / 1000 - hours * 3600 - minutes * 60)
                return `${hours}:${minutes}:${seconds}`
            }
        }
        Text{
            text: "status" + audioPlayer.mediaStatus
            horizontalAlignment: Text.AlignRight
        }
        Slider {
            id: mediaSlider
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignHCenter
            enabled: audioPlayer.seekable
            to: 1.0
            value: audioPlayer.position / audioPlayer.duration

            onMoved: {
                audioPlayer.position = (value * audioPlayer.duration)
            }
        }
        Button{
            text: "Fast Forward"
                onPressed:  {
                    var newPos = audioPlayer.position + 60000
                    if (newPos > audioPlayer.duration) {
                        newPos = audioPlayer.duration
                    }
                    audioPlayer.position = newPos
                }
}

        Text {
            id: mediaDuration

            text: {
                var hours = Math.floor(audioPlayer.duration / 3600000)
                var minutes = Math.floor(audioPlayer.duration / 60000 - hours * 60)
                var seconds = Math.floor(audioPlayer.duration / 1000 - hours * 3600 - minutes * 60)
                return `${hours}:${minutes}:${seconds}`
            }
        }
        Text {
            id: mediaStatus
            text: {
                switch (audioPlayer.mediaStatus) {
                case MediaPlayer.UnknownMediaStatus:
                    return "Unknown"
                case MediaPlayer.NoMedia:
                    return "No media"
                case MediaPlayer.LoadingMedia:
                    return "Loading"
                case MediaPlayer.LoadedMedia:
                    return "Loaded"
                case MediaPlayer.StalledMedia:
                    return "Stalled"
                case MediaPlayer.BufferingMedia:
                    return "Buffering"
                case MediaPlayer.BufferedMedia:
                    return "Buffered"
                case MediaPlayer.EndOfMedia:
                    return "End of media"
                case MediaPlayer.InvalidMedia:
                    return "Invalid"
                default:
                    return "Unknown"
                }
            }
        }
        Text {
        id : mediaError
        text: {
            switch (audioPlayer.error) {
            case MediaPlayer.NoError:
                return "No error"
            case MediaPlayer.ResourceError:
                return "Resource error"
            case MediaPlayer.FormatError:
                return "Format error"
            case MediaPlayer.NetworkError:
                return "Network error"
            case MediaPlayer.AccessDeniedError:
                return "Access denied"
            case MediaPlayer.ServiceMissingError:
                return "Service missing"
            default:
                return "Unknown error"
            }
        }
    }
}
}