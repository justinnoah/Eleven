# Current sqlite version in nuget repository.
$openssl_version = "1.0.2.6"

# Download the latest nuget
appveyor DownloadFile https://dist.nuget.org/win-x86-commandline/latest/nuget.exe
# Use it to install openssl
.\nuget install openssl.v120.dyn-rt.static -version $openssl_version

if ($env:target.Contains("i686")) {
    $platform_path = "\Win32\v120"
} else {
    $platform_path = "\x64\v120"
}

if ($env:CONFIGURATION.Contains("release")) {
    $build_type_path = "\Release"
} elseif ($env:CONFIGURATION.Contains("debug")) {
    $build_type_path = "\Debug"
}

# Set the proper environment variables for the build
# AppVeyor env variables: https://www.appveyor.com/docs/environment-variables/
$openssl_dir = $env:APPVEYOR_BUILD_FOLDER + "\openssl.v120.dyn-rt.static." + $openssl_version + "\build\native"
$openssl_lib = $openssl_dir + "\lib"

# Put it all together and set the environment variable
$OPENSSL_INCLUDES = $openssl_dir + "\include"
$OPENSSL_LIB_DIR = $openssl_lib + $platform_path + $build_type_path

$old_path = $OPENSSL_LIB_DIR + "\libeay32.lib"
$new_path = $OPENSSL_LIB_DIR + "\eay32.lib"
cp  $old_path $new_path
ls $OPENSSL_INCLUDES
ls $OPENSSL_LIB_DIR

if ($env:target.Contains("msvc")) {
     [Environment]::SetEnvironmentVariable("LIB", $OPENSSL_LIB_DIR, "Process")
     [Environment]::SetEnvironmentVariable("INCLUDE", $OPENSSL_INCLUDES, "Process")
}
if ($env:target.Contains("gnu")) {
     [Environment]::SetEnvironmentVariable("LIBRARY_PATH", $OPENSSL_LIB_DIR, "Process")
     [Environment]::SetEnvironmentVariable("C_INCLUDE_PATH", $OPENSSL_INCLUDES, "Process")
}

