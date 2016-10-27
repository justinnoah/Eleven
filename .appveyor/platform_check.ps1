if ($env:target.Contains("x86_64")) {
    Write-Host("Platform: {0}" -f $env:platform)
    if ($env:platform.Contains("x86")) {
        Write-Host("Skipping cross compilation")
        exit -1
    }
}
        
if ($env:target.Contains("i686")) {
    if ($env:platform.Contains("x64")) {
        Write-Host("Skipping cross compilation")
        exit -1
    }
}

if ($env:platform.Contains("x64")) {
    if ($env:target.Contains("gnu")) {
        Write-Host("Skipping x64 GNU builds")
        exit -1
    }
}
