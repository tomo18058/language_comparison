## C++の実行コマンド
cd "cpp"  
g++ main.cpp -O2 -o main; ./main.exe

## Object-Cの実行コマンド
ターミナルのpowershellをUbuntuに変更  
cd "objc"  
clang `gnustep-config --objc-flags` -I/usr/lib/gcc/x86_64-linux-gnu/12/include -O2 -lgnustep-base -lobjc hello.m -o hello./hello

## Rustの実行コマンド
cd "rust"  
cargo run --release



