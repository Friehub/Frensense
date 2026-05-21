window.BENCHMARK_DATA = {
  "lastUpdate": 1779394716255,
  "repoUrl": "https://github.com/Friehub/gensense",
  "entries": {
    "Gensense Engine Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "action@github.com",
            "name": "Friehub Developers",
            "username": "actions-user"
          },
          "committer": {
            "email": "action@github.com",
            "name": "Friehub Developers",
            "username": "actions-user"
          },
          "distinct": true,
          "id": "1c99bcfcb633e23edc10c24478f2aab6dfe0147a",
          "message": "fix: wrap jq command in YAML block scalar to avoid flow sequence parsing",
          "timestamp": "2026-05-21T21:06:48+01:00",
          "tree_id": "ce207342aecd48b5088d99cc5a6c6a50b173fa48",
          "url": "https://github.com/Friehub/gensense/commit/1c99bcfcb633e23edc10c24478f2aab6dfe0147a"
        },
        "date": 1779394715784,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 16696070.75,
            "range": "34289.75751623511",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 16749842.125,
            "range": "26362.666106969118",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 16664764.125,
            "range": "23649.32283014059",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 17152395,
            "range": "52669.54938992858",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 18008969.625,
            "range": "26514.817929267883",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 129361245.75,
            "range": "144907.46817737818",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 553905182,
            "range": "792628.339228034",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1084259560,
            "range": "1512435.8155488968",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2145857658,
            "range": "1466726.517060399",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 16724345.125,
            "range": "31637.015513330698",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 17126870.125,
            "range": "43503.375052660704",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 17616969.5,
            "range": "25590.416845679283",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 18365508.125,
            "range": "33293.07968392968",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 53302.598857578065,
            "range": "56.42595679526958",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 60062.286252012884,
            "range": "121.07878071690881",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5503076.65,
            "range": "12689.943824708462",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 107.28381476262736,
            "range": "0.345829453096054",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 51.413074931465125,
            "range": "0.10094207202451573",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 56.62793204222105,
            "range": "0.24553815475171606",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 52.51693353918999,
            "range": "0.09941399197245317",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 58.13537294600374,
            "range": "0.16124354092238766",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 51.45256061262823,
            "range": "0.10717170087148792",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 58.05991628601191,
            "range": "0.11285660597611608",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 55.999247206343405,
            "range": "0.04885856702649298",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 72.49189864022364,
            "range": "0.1039932692270952",
            "unit": "ns"
          }
        ]
      }
    ]
  }
}