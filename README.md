# deprecated 

このリポジトリはもう更新されません。  
PCMファイル読み込み機能は今後[pacmog](https://github.com/AkiyukiOkayasu/pacmog)で開発が進められます。  

# embedded_wav

Rustのinclude_bytes!で埋め込んだWAVEファイルを再生する。  
将来的にはマイコンのファームウェアに効果音を埋め込む想定のもの。  
no_stdで使えるように[nom](https://github.com/Geal/nom)でWAVEのパースをしている。  

| Format          | Status |
| :---            | :---: |
| WAV 16bit       | ✅ |
| WAV 24bit       | ✅ |
| WAV 32bit       | ✅ |
| WAV 32bit float | ✅ |
| WAV 64bit float | ✅ |
| IMA ADPCM | - |
| μ-law | - |
| A-law | - |
| AIFF 16bit | - |
| AIFF 24bit | - |
| AIFF 32bit float | - |




## TODO

- no_stdで動作させる
  - nomのmany0とかが使えなくなる
