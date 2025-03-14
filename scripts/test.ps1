$features = @(
    "default",
    "cache",
    "lsp-ready",
    "raw-values",
    "regex",
    "serde"
)
function Get-Combinations {
    param ( [array]$arr )

    $result = @()
    $arr | ForEach-Object {
        $result += $_
        $arr | ForEach-Object {
            $result += $arr | ForEach-Object { $_ + $_ }
        }
    }

    return $result
}

$result = Get-Combinations -arr $features

Write-Host "Combinations: $($result.Length)"
# $result

for ($i = 0; $i -lt $features.Count + 1; $i++) {
    for ($combo = 0; $combo -lt [Math]::Pow(2, $features.Count); $combo++) {
        $cargoArgs = ""
        for ($idx = 0; $idx -lt $features.Count; $idx++) {
            if ($combo -band ([Math]::Pow(2, $idx))) {
                $cargoArgs += "--features $($features[$idx]) "
            }
        }
        Write-Host "Running tests with: $cargoArgs"
        Invoke-Expression "cargo test $cargoArgs"
    }
}
