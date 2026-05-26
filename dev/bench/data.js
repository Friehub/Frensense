window.BENCHMARK_DATA = {
  "lastUpdate": 1779764440541,
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
      },
      {
        "commit": {
          "author": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "committer": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "id": "17a42266e1cd38c069a6128120578c4fe800f099",
          "message": "Merge v0.3.0",
          "timestamp": "2026-05-17T14:51:32Z",
          "url": "https://github.com/Friehub/gensense/pull/26/commits/17a42266e1cd38c069a6128120578c4fe800f099"
        },
        "date": 1779396705384,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 16967897,
            "range": "29431.648052483797",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 16996681.375,
            "range": "28451.46414488554",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 17027254.375,
            "range": "29316.190579533577",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 17595745.375,
            "range": "29154.77250739932",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 19396453.5,
            "range": "43545.196726919145",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 130129897.5,
            "range": "221283.60582143068",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 555877145.5,
            "range": "681447.4259018898",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1088168421.5,
            "range": "916743.4547245502",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2156584219,
            "range": "2746007.919448614",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 17002760.625,
            "range": "40019.26511451602",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 17424036.875,
            "range": "40554.85435500741",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 17881725.5,
            "range": "44688.157756626606",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 19347609.833333336,
            "range": "47297.41016030219",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 53024.28000855514,
            "range": "49.769529978485316",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 59488.91063504305,
            "range": "104.18710268658242",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5778892.777777778,
            "range": "10702.64210998966",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 107.10893925138356,
            "range": "0.3848670389643362",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 51.267229239805374,
            "range": "0.07999487409734499",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 54.25594100680841,
            "range": "0.15993597502397192",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 51.25577265896115,
            "range": "0.08809520398204328",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 56.96601721772885,
            "range": "0.17494456328974492",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 51.25575405116245,
            "range": "0.12050717884428032",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 56.721210458659385,
            "range": "0.10547953933927665",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 55.771605033249145,
            "range": "0.06257416052069538",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 74.13776453162673,
            "range": "0.07112382825067819",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "committer": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "id": "816cd9148f2b61a66ffa58489f0075d111dc513a",
          "message": "chore(deps): bump thiserror from 1.0.69 to 2.0.18",
          "timestamp": "2026-05-21T20:53:20Z",
          "url": "https://github.com/Friehub/gensense/pull/25/commits/816cd9148f2b61a66ffa58489f0075d111dc513a"
        },
        "date": 1779397677400,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 15968746.75,
            "range": "92384.51085984707",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 15960658.75,
            "range": "67330.2392296493",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 16049538,
            "range": "85382.74715915322",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 16596209.75,
            "range": "76697.30586335063",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 17525969.625,
            "range": "119215.67855849862",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 128695873.25,
            "range": "530856.7813754082",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 556597668,
            "range": "882917.1950250864",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1091644131.5,
            "range": "1897414.3964141607",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2172793603,
            "range": "16141553.688830137",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 15933619.5,
            "range": "73818.28203946352",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 16482025.75,
            "range": "252725.84476321936",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 16898573.875,
            "range": "108747.78144434094",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 17458764.625,
            "range": "72773.97568300366",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 47399.484715287974,
            "range": "65.16006668458789",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 53903.1331875,
            "range": "288.7485706213774",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5849307.055555556,
            "range": "20436.240403850796",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 98.54617608400454,
            "range": "0.09162473786210251",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 47.9738288248337,
            "range": "0.12592729205410186",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 49.49967835508046,
            "range": "0.09154454126788578",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 48.733344602498754,
            "range": "0.2031435002854705",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 50.46069757248998,
            "range": "0.13635211117163473",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 47.807475390596764,
            "range": "0.20785797925953406",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 50.83613787449946,
            "range": "0.15208224359586722",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 52.13088017231895,
            "range": "0.07678387158914503",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 70.40266585553974,
            "range": "0.6911521203884581",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "committer": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "id": "052d002d73763128b85d2c8502559bc0ba7af02c",
          "message": "chore(deps): bump napi-derive from 2.16.13 to 3.5.6",
          "timestamp": "2026-05-21T20:53:20Z",
          "url": "https://github.com/Friehub/gensense/pull/21/commits/052d002d73763128b85d2c8502559bc0ba7af02c"
        },
        "date": 1779397690992,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 15819388.125,
            "range": "56336.01912483573",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 15875776,
            "range": "67225.53060650826",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 15936220.125,
            "range": "69953.14390808344",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 16420782,
            "range": "58899.2491543293",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 17386041,
            "range": "98336.40845417976",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 130009123.75,
            "range": "390173.6111730337",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 557417902,
            "range": "683964.880657196",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1095319548.5,
            "range": "1912628.8373440504",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2168191662.5,
            "range": "1688335.9242260456",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 15964313.125,
            "range": "118995.69778740406",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 16283710.75,
            "range": "73768.800265342",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 16734353.625,
            "range": "75953.96730154753",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 17449609.375,
            "range": "50784.60884839296",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 47419.603634361236,
            "range": "92.79586115755887",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 54102.03936348409,
            "range": "162.17438869609603",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5839935.111111112,
            "range": "18793.107799689282",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 97.34521560438833,
            "range": "0.13632605863310068",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 47.92517607459066,
            "range": "0.12516991407756367",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 48.48851250436071,
            "range": "0.08391653919102955",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 48.24826654530345,
            "range": "0.11345397240140326",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 50.4247419860058,
            "range": "0.20518831623258485",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 48.14796643303458,
            "range": "0.08416811134305582",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 51.03920068299557,
            "range": "0.11105365831593135",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 52.45511575630772,
            "range": "0.07308666356668997",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 69.72992254574861,
            "range": "0.47660039196545895",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "committer": {
            "name": "Friehub",
            "username": "Friehub"
          },
          "id": "ee66796b30e4d3e4b800c28fe26a919d7fa82710",
          "message": "Merge v0.3.0",
          "timestamp": "2026-05-21T20:53:20Z",
          "url": "https://github.com/Friehub/gensense/pull/27/commits/ee66796b30e4d3e4b800c28fe26a919d7fa82710"
        },
        "date": 1779399574325,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 15963793.25,
            "range": "164409.77355614305",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 15945227.625,
            "range": "146080.20475655794",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 16243011.25,
            "range": "83131.7897491157",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 16896256.125,
            "range": "534535.6678850949",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 17953646.75,
            "range": "603122.2252674401",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 132671550,
            "range": "1691927.522662282",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 567524693.5,
            "range": "4812870.890754461",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1107918085,
            "range": "7098590.822374821",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2180976786.5,
            "range": "13485991.780775785",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 15909828.75,
            "range": "28125.66280066967",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 16258976.25,
            "range": "47972.30202332139",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 16797827.875,
            "range": "82732.41438120604",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 17361992.875,
            "range": "31489.49681594968",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 46044.33592132505,
            "range": "68.7027019405059",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 54113.861802549305,
            "range": "96.73507035083597",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5812707.333333334,
            "range": "26639.603427053647",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 97.6458692309728,
            "range": "0.17434194670631625",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 47.77579819349853,
            "range": "0.10470557675151637",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 48.53185262741818,
            "range": "0.09048665406396181",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 48.15248852878052,
            "range": "0.08869498259636514",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 51.78934347439273,
            "range": "0.150581517431754",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 48.16841511693809,
            "range": "0.09830633292044087",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 50.94330973027584,
            "range": "0.13747016741243218",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 52.9979563773272,
            "range": "0.63229067334566",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 69.95444618725868,
            "range": "0.08432343865486956",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f16652f8d81d8915f910bdc5f138b7727fdbc7e2",
          "message": "docs: add MCP server docs, changelog page, and benchmark CI fix (#29)\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-21T23:26:48Z",
          "tree_id": "4ac27087f6b75ea70663329907a15fde1bd4deb5",
          "url": "https://github.com/Friehub/gensense/commit/f16652f8d81d8915f910bdc5f138b7727fdbc7e2"
        },
        "date": 1779406748845,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 15928194.375,
            "range": "48195.43331936002",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 15971393.625,
            "range": "55465.36229029298",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 15988237.25,
            "range": "38079.283048957586",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 16414748.875,
            "range": "38756.27526193857",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 17337657,
            "range": "36889.49657008052",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 131353705.5,
            "range": "1251900.004774332",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 555855060.5,
            "range": "838780.1938086748",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1092315687.5,
            "range": "1227412.6423090696",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2160955818,
            "range": "2483705.5908054113",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 16045928.875,
            "range": "57979.481195658445",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 16504483.5,
            "range": "52673.255889862776",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 16881855,
            "range": "48312.0027422905",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 17579163.125,
            "range": "53908.07634294033",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 48063.94111394558,
            "range": "76.54826138653385",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 54794.131345177666,
            "range": "77.2399996146569",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5822132.444444444,
            "range": "18662.14480201342",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 96.13413979266195,
            "range": "0.1533409044121352",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 47.875170377149864,
            "range": "0.22730275780952694",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 48.787819185750614,
            "range": "0.05717807014162244",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 48.190089072767094,
            "range": "0.10670957121422003",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 50.807449651804916,
            "range": "0.09931978277405032",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 48.11000969402316,
            "range": "0.08281393447915152",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 51.0779261947409,
            "range": "0.11516748775020369",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 53.34336672903205,
            "range": "0.08880885888902858",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 68.92575388648396,
            "range": "0.15344745614485875",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "5e678cadc86d322b57f26c9a8dc87fc682396e03",
          "message": "V0.3.1 docs (#31)\n\n* docs: add MCP server docs, changelog page, and benchmark CI fix\n\n* fix: add keep_files to deploy-docs to preserve benchmark dashboard\n\n* fix: add --force to cargo-criterion install to fix cached binary conflict\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-22T00:11:07Z",
          "tree_id": "d5de1eedd2a6c94b3b5acbba8f18b55c2e2e4519",
          "url": "https://github.com/Friehub/gensense/commit/5e678cadc86d322b57f26c9a8dc87fc682396e03"
        },
        "date": 1779409398885,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 20612069.833333336,
            "range": "64772.81605005725",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 20487193.666666668,
            "range": "60780.91562092212",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 20362446.5,
            "range": "46024.350982904434",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 20939040.333333332,
            "range": "50632.27170109472",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 22034123.166666664,
            "range": "51822.79947996416",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 132966648,
            "range": "176976.47625803947",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 559739535.5,
            "range": "492621.2693542242",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1090684643,
            "range": "1748478.3334583044",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2157445864,
            "range": "2825768.091532588",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 20155525,
            "range": "44324.55011308193",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 20439384.833333336,
            "range": "38679.79781329816",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 20962612.333333332,
            "range": "54319.99203562829",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 21679498.833333332,
            "range": "44926.238602397985",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 53067.83350131657,
            "range": "77.73328215466195",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 59451.20078218283,
            "range": "117.9147467775984",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5766322.222222222,
            "range": "17599.202987550998",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 102.9912914258858,
            "range": "0.554411492372591",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 51.2012062359332,
            "range": "0.10868769708013674",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 53.884281372746656,
            "range": "0.3494682973956434",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 51.237960188264445,
            "range": "0.1032230521166596",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 57.09034434008488,
            "range": "0.15523695757610073",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 51.16354112843039,
            "range": "0.10899730162234245",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 56.790638538539,
            "range": "0.10679806274542267",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 65.45649669997806,
            "range": "0.5224567378964372",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 73.92880055068692,
            "range": "0.10391246889648137",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "102c1985cd89148d4275e9baaecccd6ed7a6e713",
          "message": "V0.3.1 docs (#32)\n\n* docs: add v0.4.0 project memory spec\n\n* fix: add --bin gensense to generate-docs step to resolve ambiguous binary\n\n* docs: add MED-07 for post_process_ngrams benchmark gap\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-22T00:27:40Z",
          "tree_id": "64021bb60d9a9087729716b0732342e64b9c124e",
          "url": "https://github.com/Friehub/gensense/commit/102c1985cd89148d4275e9baaecccd6ed7a6e713"
        },
        "date": 1779410382199,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 18415599,
            "range": "45853.8519859314",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 18273039.5,
            "range": "35879.66066300869",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 18306261.375,
            "range": "30207.60381370783",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 19509607.5,
            "range": "39860.4415923357",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 20399900.833333336,
            "range": "30378.473460677047",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 131221300.5,
            "range": "131820.55820971727",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 556737143,
            "range": "599725.0327527523",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1085437783.5,
            "range": "1346902.7871876955",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2149578559,
            "range": "2962724.7467011213",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 18144541.5,
            "range": "32533.617847412825",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 18441707.5,
            "range": "28079.331551492214",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 19577578.666666668,
            "range": "42051.230153440505",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 20274212,
            "range": "31267.2921448946",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 47301.35235997483,
            "range": "72.91108048120304",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 54240.11586204833,
            "range": "197.19510245132577",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5776802.722222222,
            "range": "11247.662333646796",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 98.5094451776437,
            "range": "0.0877310735492661",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 48.06624556743924,
            "range": "0.06237054800414911",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 48.353935020027905,
            "range": "0.11600618929421146",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 48.170595359613735,
            "range": "0.10530549379280789",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 50.43309389168153,
            "range": "0.09862071637250631",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 48.079644138005676,
            "range": "0.1070946098598539",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 50.82436466821441,
            "range": "0.16383548583645527",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 52.902432598132926,
            "range": "0.360555909060106",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 69.80921069720456,
            "range": "0.2669884325562052",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0de503eb4672e7ab38d8b3a9ab8156bc93285944",
          "message": "V0.3.1 docs (#33)\n\n* docs: add AtomicSection CSA constraint proposal for v0.4.0\n\n* docs: add SRI diff-only baselines (v0.4.0) + v0.5.0 roadmap (hallucination, secrets, perf)\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-22T07:30:01Z",
          "tree_id": "258aa81a3b86b891ff6470dce8bd0525be434a11",
          "url": "https://github.com/Friehub/gensense/commit/0de503eb4672e7ab38d8b3a9ab8156bc93285944"
        },
        "date": 1779435738450,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 20629464,
            "range": "82873.38492870146",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 20566004,
            "range": "148786.3203585148",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 20443647.666666668,
            "range": "108454.65907454399",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 21023494.833333336,
            "range": "110234.52034294514",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 22151086,
            "range": "171782.92855024338",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 133441593,
            "range": "348153.0214190483",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 559872321,
            "range": "1169295.4646408558",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1094109102.5,
            "range": "1137645.661702752",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2162913751,
            "range": "2691623.928514123",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 20560693.333333332,
            "range": "465903.8294285545",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 20450695.333333336,
            "range": "53196.6754555693",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 20964163,
            "range": "84912.70119249912",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 21759739,
            "range": "94877.996915577",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 52979.21912385644,
            "range": "47.26696668642456",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 59857.42829861111,
            "range": "173.62036411762182",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 5798128.444444444,
            "range": "17657.84805317781",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 105.19523605775785,
            "range": "0.6864987252743627",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 51.250711989571066,
            "range": "0.09869572448595174",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 54.30474655255584,
            "range": "0.12792291396078104",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 51.1618758444441,
            "range": "0.09875777171867792",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 56.943022985227,
            "range": "0.14478664039579373",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 51.21469484232617,
            "range": "0.1116053385867745",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 56.76601007278509,
            "range": "0.09585699796949915",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 55.80858657972601,
            "range": "0.06898766351317208",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 72.79155008311648,
            "range": "0.1873264409584407",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0d90a54865967f45bd110ace3368e8a4b7979e7c",
          "message": "V0.3.1 tasks (#34)\n\n* fix: CRIT-01 — Engine::run() and run_detailed() now return Err for invalid paths\n\n* fix: MED-03 — hermetic MCP tests with clear error when binary missing\n\n* feat: MED-04 — MCP streaming for large scans with progress notifications\n\n* feat: MED-06 — MCP ping health-check method\n\n* v0.3.1 — unified crate, MCP filters, clippy pedantic, licensing\n\n- Consolidate into single crate: 'cargo install gensense' produces both\n  'gensense' (CLI) and 'gensense-mcp' (MCP server) binaries\n- MCP: language and rules filter params applied server-side post-scan\n- MCP: 36/36 tests pass, includes streaming and ping health-check\n- Clippy: all ~35 pedantic violations fixed, 4 -A flags removed\n- License: 13 files attributed, solidity.rs changed to MIT, 100% consistent\n- Dedup: RulesWrapper and is_in_async_scope extracted to shared modules\n- Pre-commit hook: runs full test suite, no more suppressed lints\n- Version bumped to 0.3.1 across all Cargo.toml and package.json\n\n* fix: filter collect_files to supported extensions to prevent binary file crash\n\n* docs: add GenSense article and unignore it in .gitignore\n\nPublished alongside the Friehub engineering blog post.\nArchive repo referenced: github.com/Friehub/Friehub-auditor\n\n* feat: add branded CLI header with tagline for product screenshots\n\n* feat: add detailed description to --help output\n\n* docs: document known bottlenecks and resolutions in V0_3_1_ISSUES.md\n\n* feat: add exclude_scope field to rule DSL to filter test-context false positives\n\n* feat: extend ReachabilityChecker to all CSA content constraint checks\n\n* v0.3.1-tasks: BTL-04/05, exclude_scope, dead code, corpus, report\n\n* feat: rule quality pipeline — precision tiers, --suite flag, and precision metadata for all rules\n\n* fix: bump self-audit warning threshold to 165 (baseline debt from --suite flag)\n\n* docs: add historical self-scan benchmark script + BENCHMARK.md section\n\n* fix: MCP tests scan temp dirs instead of CWD; CI baseline regression emits before comparing\n\n* chore: remove stray benchmark CSV\n\n* docs: update BENCHMARK.md with v0.3.1 criterion and tokio data\n\n* fix: mkdir -p baseline dir before emit in CI\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-23T20:27:10Z",
          "tree_id": "0e577486a84b951e71751ec6c352af2d4359a0ec",
          "url": "https://github.com/Friehub/gensense/commit/0d90a54865967f45bd110ace3368e8a4b7979e7c"
        },
        "date": 1779568882145,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 17908400.125,
            "range": "138472.24299162626",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 17670963.125,
            "range": "39828.93634289503",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 17442038.625,
            "range": "50916.93089604378",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 18020909.5,
            "range": "63461.76524832845",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 19982379.166666664,
            "range": "90491.23069345675",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 29956122.96818182,
            "range": "6731251.396029628",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 207634531.8333333,
            "range": "356326.3479739133",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1143526763,
            "range": "1775997.6128697395",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2263509018,
            "range": "1915037.3210012913",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 17501377.25,
            "range": "107782.42353647947",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 17870409.5,
            "range": "70510.41617318988",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 18359760.625,
            "range": "72448.3596637845",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 19998748.666666664,
            "range": "100307.03091919054",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 53578.58229234263,
            "range": "67.94831899268104",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 60467.346408655845,
            "range": "111.64495555862666",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 6865711.8125,
            "range": "30255.417662858963",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 119.63237546792054,
            "range": "0.28744237186027743",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 50.94683100519369,
            "range": "0.11331784316859927",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 54.31571965542683,
            "range": "0.1363443089149678",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 51.128522874186764,
            "range": "0.10309413004040861",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 56.667776269028096,
            "range": "0.10993492798428325",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 51.150091689493244,
            "range": "0.10421634821157214",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 56.6699726458937,
            "range": "0.1397040801366168",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 54.75294666756058,
            "range": "0.07132397285140504",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 72.60015293282419,
            "range": "0.09977445778968133",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/10",
            "value": 31403.26230654762,
            "range": "57.134737908068146",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/50",
            "value": 518092.24475524476,
            "range": "1513.8214280612951",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/200",
            "value": 6456720.625,
            "range": "13917.721927911043",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/500",
            "value": 37090915.5,
            "range": "141516.0207375884",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "da8dc52660e591b4c276e96bc0de339dd6c2952f",
          "message": "V0.3.1 tasks (#37)\n\n* fix: bump self-audit threshold 165→175\n\n* fix: remove file-cache skip from full scan path\n\ncollect_and_snapshot_files() was skipping files whose blake3 hash\nmatched the previous run's cache, causing audit() (including Phase 3\nfile_check for LONG_FILE) to never be called for cached files. This\nmade both --json and text output return 0 advisories on subsequent\nruns, making JSON appear broken.\n\nThe cache is still maintained and used by run_files() for diff-only\nmode where skipping unchanged files is intentional.\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-24T06:54:45Z",
          "tree_id": "00dd95c2e796f6ed30ecfd92d9d57c9c0225c5ec",
          "url": "https://github.com/Friehub/gensense/commit/da8dc52660e591b4c276e96bc0de339dd6c2952f"
        },
        "date": 1779606545411,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 16658111.625,
            "range": "30809.724728018045",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 16445149.25,
            "range": "33844.2362241447",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 16233480.375,
            "range": "38201.78287178278",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 16778773.625,
            "range": "47382.59788379073",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 17828074.625,
            "range": "45022.11340069771",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 136239185.5,
            "range": "201661.39516979456",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 586647340,
            "range": "1312945.317390561",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1150796881.5,
            "range": "2053300.8880466223",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2273966847,
            "range": "3059881.74687624",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 16197311.375,
            "range": "32917.24059060216",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 16585475.25,
            "range": "32618.496695905924",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 17144262,
            "range": "29046.357384324074",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 18286869.625,
            "range": "31240.975995361805",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 47602.780806412135,
            "range": "112.24273284942979",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 54458.74315807649,
            "range": "93.33486185890473",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 6821600.8125,
            "range": "18129.788453131914",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 111.0115174749507,
            "range": "0.1914562133773519",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 47.257503751322865,
            "range": "0.09781836075506482",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 48.881817132043984,
            "range": "0.11037973530715137",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 47.29471852438046,
            "range": "0.09665745655030229",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 49.31362857642374,
            "range": "0.11344269185465086",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 46.351535383876254,
            "range": "0.28720134083760335",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 50.44467463530653,
            "range": "0.13517134482744736",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 52.54335169428299,
            "range": "0.16142882007804624",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 69.7189548184158,
            "range": "0.24958459750181583",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/10",
            "value": 29313.040839460784,
            "range": "50.65482681436643",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/50",
            "value": 478532.01839857316,
            "range": "859.630005556458",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/200",
            "value": 5892860.222222222,
            "range": "9563.346396882791",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/500",
            "value": 33731577.5,
            "range": "59362.56164610386",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "87064f4fccbbdc48ec3685ab56368591ea173d9e",
          "message": "V0.3.1 tasks (#38)\n\n* fix: bump self-audit threshold 165→175\n\n* fix: remove file-cache skip from full scan path\n\ncollect_and_snapshot_files() was skipping files whose blake3 hash\nmatched the previous run's cache, causing audit() (including Phase 3\nfile_check for LONG_FILE) to never be called for cached files. This\nmade both --json and text output return 0 advisories on subsequent\nruns, making JSON appear broken.\n\nThe cache is still maintained and used by run_files() for diff-only\nmode where skipping unchanged files is intentional.\n\n* fix: move corpus targets out of tests/ path so exclude_scope doesn't block them\n\nAll Rust rules in core.yml use exclude_scope='tests/|...' which matched\nany file path containing tests/. Corpus fixtures under tests/corpus/targets/\nwere silently skipped, making positive fixtures like RUST_CLONE_IN_LOOP\nappear as 'not currently firing'.\n\n- Move tests/corpus/targets/ -> corpus/targets/\n- Rename test() -> clone_in_loop_case() in the positive fixture\n- Update CI workflow and README paths\n- Regenerate baseline (8 findings, up from 4)\n\n* fix: invalidate file cache when language filter changes (BUG-3)\n\n* feat: add GENSENSE_BENCH_QUICK env var for fast CI benchmarks (MED-01)\n\n* feat: add native TypeScript rule TS_TAUTOLOGICAL_ASSERT (IMP-1)\n\n* fix: uncomment NAPI integration test assertions (IMP-2)\n\n* fix: tighten NAPI version check to exact crate version (0.3.1)\n\n* docs: add deferred --severity pre-filter item to v0.4.0 project memory\n\n* refactor: move temporal analyzer to dedicated src/temporal/ folder behind feature flag\n\n- New src/temporal/ folder with analyzer.rs, config.rs, handler.rs, mod.rs\n- Added 'temporal' Cargo feature flag (on by default)\n- Call sites in ir.rs and compiler.rs reduced to 1-line delegations\n- Deleted src/semantics/temporal.rs\n\n- CSA regex narrowed: \\b(validate|verify|check) -> \\b(validate|verify)\n- Suppression paths changed from **/*.rs to src/**/*.rs\n- Removed 14 style/noise YAML rules (self-audit 186 -> 69)\n- 6 new CSA corpus fixtures and 3 test functions\n\n- CHANGELOG.md and docs/changelog.md synced for unreleased v0.3.1\n- Removed superseded bug documents (V0_3_1_ISSUES.md, V0_3_1_REPORT.md, AUDIT_V0.3.0_REPORT.md)\n- Added FEATURE_MAP.md with file ownership for each differentiator\n- Restructured GAP_ANALYSIS.md as phased build plan (P0-P5)\n\n* remove RUST_SQL_COLUMN_MUST_EXIST_IN_PRISMA rule\n\nThis rule fires on any double-quoted camelCase identifier in Rust source\n(including #[doc] strings) when no Prisma schema is found. False positives\noutweigh the value for the general case.\n\nRemoved from cross-layer-contracts.yml, test YAML, assertion, and corpus README.\n\n* fix: filter spawn_blocking wrappers and builder excludes in ASYNC_BLOCKING_IO\n\n- Skip blocking calls inside closures passed to spawn_blocking/asyncify\n  (tokio wraps std::fs calls in asyncify() — was producing 19 false positives)\n- Exclude DirBuilder, DirBuilderExt, OpenOptions from matching (non-I/O constructors)\n  (was producing 2 false positives on builder/setter calls)\n- 21 → 0 false positives on tokio/src\n\n* fix: add asyncify exclusion to BLOCKING_IN_ASYNC and exclude_scope to CSA_VALIDATE\n\n- RUST_BLOCKING_IN_ASYNC: add asyncify to must_not_contain pattern\n  (tokio uses asyncify() instead of spawn_blocking() — eliminates 18 FP on tokio/src/fs/)\n- RUST_CSA_VALIDATE_UNCONDITIONAL: add exclude_scope to skip test files\n  (was firing on src/sync/tests/ — test code is not production code)\n- Total tokio/src findings: 653 → 611\n\n* remove Solidity rules, feature flag, and all source references\n\n- Deleted solidity/core.yml (7 rules) and solidity/security.yml (2 rules)\n- Removed SOL_CSA_VALIDATE_UNCONDITIONAL and SOL_CSA_SANITIZE_PASSTHROUGH from csa.yml\n- Removed tree-sitter-solidity dep and solidity feature from Cargo.toml\n- Cleaned up parser.rs, gensense.rs, gensense-mcp.rs, README.md, docs/guide.md\n- Solidty not in default features and was dead code (not compiled in binary)\n- Updated precision count from 75 to 60 rules across changelogs\n\n* wire temporal analyzer: RUST_LOCK_SLEEP rule + event discovery + MustFollow fix\n\n- Created RUST_LOCK_SLEEP (must_not_follow: lock → sleep) — detects\n  holding a mutex while calling thread::sleep(), a classic deadlock pattern.\n- Called discover_events() in run_detailed, run_files, and run_content.\n  Previously defined but never called — temporal analysis always returned\n  empty because the SymbolRegistry had no TemporalEvent entries.\n- Fixed MustFollow bug: check_must_follow now requires current_step > 0\n  before firing, preventing false positives on scopes where no event in\n  the sequence is present.\n- Added compile tests for the on-disk YAML rule.\n\n* detect object spread overriding security properties\n\n- Fix propagate_object_taint: add two-pass detection of spread_element\n  children. First pass resolves taint on each spread source; if the spread\n  is tainted, the second pass marks every explicit property in the object as\n  overwritable via taint_field. Previously spread_element was silently\n  ignored, so explicit security properties like isAdmin: false were marked\n  safe even when ...prefs (user-controlled) could override them.\n\n- New YAML rule TS_OBJECT_SPREAD_OVERRIDE_SECURITY: heuristic regex check\n  on object literals that contain both a spread element and a security\n  property name (isAdmin, role, permissions, scopes, etc.). Catches the\n  prototype-pollution-to-privilege-escalation pattern without requiring\n  full taint tracking.\n\n* track GAP_ANALYSIS.md (was gitignored by *.md)\n\n* fix: correct FNV-1a prime, TS symbol query, paper discrepancies, and YAML rule loading\n\n- Fix FNV-1a prime constant in ir.rs (was missing a zero digit)\n- Add method_definition to TS symbol extraction query\n- Fix GENSENSE_PAPER.md: BFS->DFS, Suite enum values, benchmark file counts,\n  GenSenseEnvironment variants, CI threshold, rule loading priority, --category\n- Fix TS YAML rule loading (duplicate keys in core.yml)\n- Fix must_not_contain bypass to use full body text\n- Fix is_rule_enabled signature with severity_filter parameter\n- Add TS_UNHANDLED_ASYNC_REJECTION, RUST_MISSING_AWAIT, RUST_DISCARDED_RESULT rules\n- Remove deprecated TS rule tests (SQL_INJECTION, PATH_TRAVERSAL, OPEN_REDIRECT)\n\n* feat: expose all tuning parameters via Engine + CLI flags\n\n* feat: SPG foundation — graph in context, taint edges, algebraic combinators\n\n* docs(CAPABILITIES): add SPG capabilities — graph in context, taint edges, algebraic combinators (57 total)\n\n* docs(GAP_ANALYSIS): add SPG Phase 6 — graph in context, taint edges, algebraic combinators\n\n* v0.3.1: CLI rule modification, CSA delegation test, docs update\n\nCLI:\n  - --disable-rule <id> suppresses specific rules (repeatable)\n  - --override-severity <RULE_ID>:<level> changes severity (repeatable)\n  - --help rewritten with all 21 flags in categorized sections + 10 examples\n  - gensense with no path defaults to current directory\n\nDocs:\n  - README rewritten: 357->250 lines, v0.3.1 features, deduped suppression\n  - VitePress: index, guide, extending, authoring, changelog all updated\n  - extending.md: new CSA and Algebraic Flow Combinators sections\n  - Removed all 'parallel via Rayon' lies (Rayon was removed in v0.3.0)\n  - Fixed '17 rules support auto-fix' lie (only 1 YAML rule has fix_pattern)\n\nTests:\n  - CSA delegation suppression test (body_may_delegate_via)\n  - All 16 test suites green\n\nEngine:\n  - Engine stores disabled_rule_ids + severity_overrides\n  - Merged with config file values (CLI wins on conflict)\n  - Applied in initialize_auditor_and_config and all audit paths\n\n* chore: remove demo.cast from tracking, add to gitignore\n\n* chore: add .semgrepignore to exclude test fixtures from scans\n\n* fix: clear file cache between baseline emit/compare steps in CI\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-26T02:23:17Z",
          "tree_id": "a2d03e38e099de516848c1e479692b3d3760694b",
          "url": "https://github.com/Friehub/gensense/commit/87064f4fccbbdc48ec3685ab56368591ea173d9e"
        },
        "date": 1779763066960,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 13353346.6,
            "range": "51081.20297312709",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 12801645.5,
            "range": "38148.48340272876",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 13890073.2,
            "range": "57103.37580621132",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 15761532.75,
            "range": "140469.1198311746",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 18035379.625,
            "range": "52000.711476802826",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 70311895,
            "range": "297082.04838574154",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 670573715.5,
            "range": "1029111.930629611",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1339974277.5,
            "range": "4922790.111503005",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2755567425.5,
            "range": "2960350.362843275",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 12701976.6,
            "range": "161305.39453625816",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 13085787.5,
            "range": "79765.21292388494",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 14079273.3,
            "range": "101832.52745211069",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 16521930.125,
            "range": "156670.4163685441",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 53013.245577698166,
            "range": "103.39818273781748",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 59467.9288213628,
            "range": "122.42221158348627",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 8015027,
            "range": "24128.14967164011",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 211.11506201051532,
            "range": "0.2589883086312121",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 56.2268492215299,
            "range": "0.10592378338684318",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 50.47267704643268,
            "range": "0.2056247205189192",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 56.25604986006078,
            "range": "0.1053316675314861",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 54.34572162705965,
            "range": "0.11067643475088772",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 56.1667132364189,
            "range": "0.10099114557471131",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 53.51254040361982,
            "range": "0.11001364901423191",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 55.442607857142136,
            "range": "0.09642965979539653",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 73.58466083340639,
            "range": "0.08734241760475107",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/10",
            "value": 31484.333727890422,
            "range": "44.25241687823477",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/50",
            "value": 509392.65065681445,
            "range": "1286.5906974073096",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/200",
            "value": 6314097.375,
            "range": "10745.513959228992",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/500",
            "value": 36476973.5,
            "range": "139240.22977799177",
            "unit": "ns"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76975899+0xademola@users.noreply.github.com",
            "name": "0xademola",
            "username": "0xademola"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": false,
          "id": "82eb8dbbf63b295267a37702206c425b6a7250dd",
          "message": "V0.3.1 tasks (#39)\n\n* fix: bump self-audit threshold 165→175\n\n* fix: remove file-cache skip from full scan path\n\ncollect_and_snapshot_files() was skipping files whose blake3 hash\nmatched the previous run's cache, causing audit() (including Phase 3\nfile_check for LONG_FILE) to never be called for cached files. This\nmade both --json and text output return 0 advisories on subsequent\nruns, making JSON appear broken.\n\nThe cache is still maintained and used by run_files() for diff-only\nmode where skipping unchanged files is intentional.\n\n* fix: move corpus targets out of tests/ path so exclude_scope doesn't block them\n\nAll Rust rules in core.yml use exclude_scope='tests/|...' which matched\nany file path containing tests/. Corpus fixtures under tests/corpus/targets/\nwere silently skipped, making positive fixtures like RUST_CLONE_IN_LOOP\nappear as 'not currently firing'.\n\n- Move tests/corpus/targets/ -> corpus/targets/\n- Rename test() -> clone_in_loop_case() in the positive fixture\n- Update CI workflow and README paths\n- Regenerate baseline (8 findings, up from 4)\n\n* fix: invalidate file cache when language filter changes (BUG-3)\n\n* feat: add GENSENSE_BENCH_QUICK env var for fast CI benchmarks (MED-01)\n\n* feat: add native TypeScript rule TS_TAUTOLOGICAL_ASSERT (IMP-1)\n\n* fix: uncomment NAPI integration test assertions (IMP-2)\n\n* fix: tighten NAPI version check to exact crate version (0.3.1)\n\n* docs: add deferred --severity pre-filter item to v0.4.0 project memory\n\n* refactor: move temporal analyzer to dedicated src/temporal/ folder behind feature flag\n\n- New src/temporal/ folder with analyzer.rs, config.rs, handler.rs, mod.rs\n- Added 'temporal' Cargo feature flag (on by default)\n- Call sites in ir.rs and compiler.rs reduced to 1-line delegations\n- Deleted src/semantics/temporal.rs\n\n- CSA regex narrowed: \\b(validate|verify|check) -> \\b(validate|verify)\n- Suppression paths changed from **/*.rs to src/**/*.rs\n- Removed 14 style/noise YAML rules (self-audit 186 -> 69)\n- 6 new CSA corpus fixtures and 3 test functions\n\n- CHANGELOG.md and docs/changelog.md synced for unreleased v0.3.1\n- Removed superseded bug documents (V0_3_1_ISSUES.md, V0_3_1_REPORT.md, AUDIT_V0.3.0_REPORT.md)\n- Added FEATURE_MAP.md with file ownership for each differentiator\n- Restructured GAP_ANALYSIS.md as phased build plan (P0-P5)\n\n* remove RUST_SQL_COLUMN_MUST_EXIST_IN_PRISMA rule\n\nThis rule fires on any double-quoted camelCase identifier in Rust source\n(including #[doc] strings) when no Prisma schema is found. False positives\noutweigh the value for the general case.\n\nRemoved from cross-layer-contracts.yml, test YAML, assertion, and corpus README.\n\n* fix: filter spawn_blocking wrappers and builder excludes in ASYNC_BLOCKING_IO\n\n- Skip blocking calls inside closures passed to spawn_blocking/asyncify\n  (tokio wraps std::fs calls in asyncify() — was producing 19 false positives)\n- Exclude DirBuilder, DirBuilderExt, OpenOptions from matching (non-I/O constructors)\n  (was producing 2 false positives on builder/setter calls)\n- 21 → 0 false positives on tokio/src\n\n* fix: add asyncify exclusion to BLOCKING_IN_ASYNC and exclude_scope to CSA_VALIDATE\n\n- RUST_BLOCKING_IN_ASYNC: add asyncify to must_not_contain pattern\n  (tokio uses asyncify() instead of spawn_blocking() — eliminates 18 FP on tokio/src/fs/)\n- RUST_CSA_VALIDATE_UNCONDITIONAL: add exclude_scope to skip test files\n  (was firing on src/sync/tests/ — test code is not production code)\n- Total tokio/src findings: 653 → 611\n\n* remove Solidity rules, feature flag, and all source references\n\n- Deleted solidity/core.yml (7 rules) and solidity/security.yml (2 rules)\n- Removed SOL_CSA_VALIDATE_UNCONDITIONAL and SOL_CSA_SANITIZE_PASSTHROUGH from csa.yml\n- Removed tree-sitter-solidity dep and solidity feature from Cargo.toml\n- Cleaned up parser.rs, gensense.rs, gensense-mcp.rs, README.md, docs/guide.md\n- Solidty not in default features and was dead code (not compiled in binary)\n- Updated precision count from 75 to 60 rules across changelogs\n\n* wire temporal analyzer: RUST_LOCK_SLEEP rule + event discovery + MustFollow fix\n\n- Created RUST_LOCK_SLEEP (must_not_follow: lock → sleep) — detects\n  holding a mutex while calling thread::sleep(), a classic deadlock pattern.\n- Called discover_events() in run_detailed, run_files, and run_content.\n  Previously defined but never called — temporal analysis always returned\n  empty because the SymbolRegistry had no TemporalEvent entries.\n- Fixed MustFollow bug: check_must_follow now requires current_step > 0\n  before firing, preventing false positives on scopes where no event in\n  the sequence is present.\n- Added compile tests for the on-disk YAML rule.\n\n* detect object spread overriding security properties\n\n- Fix propagate_object_taint: add two-pass detection of spread_element\n  children. First pass resolves taint on each spread source; if the spread\n  is tainted, the second pass marks every explicit property in the object as\n  overwritable via taint_field. Previously spread_element was silently\n  ignored, so explicit security properties like isAdmin: false were marked\n  safe even when ...prefs (user-controlled) could override them.\n\n- New YAML rule TS_OBJECT_SPREAD_OVERRIDE_SECURITY: heuristic regex check\n  on object literals that contain both a spread element and a security\n  property name (isAdmin, role, permissions, scopes, etc.). Catches the\n  prototype-pollution-to-privilege-escalation pattern without requiring\n  full taint tracking.\n\n* track GAP_ANALYSIS.md (was gitignored by *.md)\n\n* fix: correct FNV-1a prime, TS symbol query, paper discrepancies, and YAML rule loading\n\n- Fix FNV-1a prime constant in ir.rs (was missing a zero digit)\n- Add method_definition to TS symbol extraction query\n- Fix GENSENSE_PAPER.md: BFS->DFS, Suite enum values, benchmark file counts,\n  GenSenseEnvironment variants, CI threshold, rule loading priority, --category\n- Fix TS YAML rule loading (duplicate keys in core.yml)\n- Fix must_not_contain bypass to use full body text\n- Fix is_rule_enabled signature with severity_filter parameter\n- Add TS_UNHANDLED_ASYNC_REJECTION, RUST_MISSING_AWAIT, RUST_DISCARDED_RESULT rules\n- Remove deprecated TS rule tests (SQL_INJECTION, PATH_TRAVERSAL, OPEN_REDIRECT)\n\n* feat: expose all tuning parameters via Engine + CLI flags\n\n* feat: SPG foundation — graph in context, taint edges, algebraic combinators\n\n* docs(CAPABILITIES): add SPG capabilities — graph in context, taint edges, algebraic combinators (57 total)\n\n* docs(GAP_ANALYSIS): add SPG Phase 6 — graph in context, taint edges, algebraic combinators\n\n* v0.3.1: CLI rule modification, CSA delegation test, docs update\n\nCLI:\n  - --disable-rule <id> suppresses specific rules (repeatable)\n  - --override-severity <RULE_ID>:<level> changes severity (repeatable)\n  - --help rewritten with all 21 flags in categorized sections + 10 examples\n  - gensense with no path defaults to current directory\n\nDocs:\n  - README rewritten: 357->250 lines, v0.3.1 features, deduped suppression\n  - VitePress: index, guide, extending, authoring, changelog all updated\n  - extending.md: new CSA and Algebraic Flow Combinators sections\n  - Removed all 'parallel via Rayon' lies (Rayon was removed in v0.3.0)\n  - Fixed '17 rules support auto-fix' lie (only 1 YAML rule has fix_pattern)\n\nTests:\n  - CSA delegation suppression test (body_may_delegate_via)\n  - All 16 test suites green\n\nEngine:\n  - Engine stores disabled_rule_ids + severity_overrides\n  - Merged with config file values (CLI wins on conflict)\n  - Applied in initialize_auditor_and_config and all audit paths\n\n* chore: remove demo.cast from tracking, add to gitignore\n\n* chore: add .semgrepignore to exclude test fixtures from scans\n\n* fix: clear file cache between baseline emit/compare steps in CI\n\n* chore: raise benchmark alert threshold to 125% to account for SPG/CSA/temporal overhead\n\n---------\n\nCo-authored-by: Friehub Developers <action@github.com>",
          "timestamp": "2026-05-26T02:46:53Z",
          "tree_id": "d8a76cfcea20c7d90a87aec6176a0346a3a243ef",
          "url": "https://github.com/Friehub/gensense/commit/82eb8dbbf63b295267a37702206c425b6a7250dd"
        },
        "date": 1779764439839,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "scan_throughput/rust_clean_service",
            "value": 9678895.42857143,
            "range": "26880.172922782323",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/rust_service_with_violations",
            "value": 9271236.5,
            "range": "54086.83553976593",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_clean_service",
            "value": 10517029.5,
            "range": "131967.95335709956",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_service_with_violations",
            "value": 11342279.583333332,
            "range": "58736.16315722189",
            "unit": "ns"
          },
          {
            "name": "scan_throughput/ts_mixed_real_world",
            "value": 13557096.4,
            "range": "125570.13911068495",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/10",
            "value": 60855549.3,
            "range": "278415.37341714127",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/50",
            "value": 633631557.5,
            "range": "2164315.7501757145",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/100",
            "value": 1267969876.5,
            "range": "5018794.390198588",
            "unit": "ns"
          },
          {
            "name": "project_scale/files_scanned/200",
            "value": 2584711265,
            "range": "7086203.699594736",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/5",
            "value": 9230890.92857143,
            "range": "66050.99372736005",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/20",
            "value": 9588665.642857142,
            "range": "54451.66103328957",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/50",
            "value": 10670498.416666668,
            "range": "52105.1112249512",
            "unit": "ns"
          },
          {
            "name": "taint_analysis/chain_depth/100",
            "value": 12621870.9,
            "range": "67594.10495996558",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_model_names_20_models",
            "value": 24578.463188834154,
            "range": "43.52995767661451",
            "unit": "ns"
          },
          {
            "name": "schema_contract/extract_field_names_20_models",
            "value": 30724.21076190476,
            "range": "545.1499725489249",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/compile_all_builtin_rules",
            "value": 7833056.714285715,
            "range": "13447.07586126678",
            "unit": "ns"
          },
          {
            "name": "rule_compilation/engine_cold_start",
            "value": 98.48773005978168,
            "range": "0.13065681868646314",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/1000",
            "value": 48.07610810169761,
            "range": "0.07383889924847195",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/1000",
            "value": 49.960922128528374,
            "range": "0.041304979199782495",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/10000",
            "value": 48.056980360973384,
            "range": "0.06700255312957347",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/10000",
            "value": 49.26104235112529,
            "range": "0.02976310157538405",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/start/100000",
            "value": 49.5154349229566,
            "range": "0.0686040416051104",
            "unit": "ns"
          },
          {
            "name": "symbol_registry/find_function_at/middle/100000",
            "value": 49.579212881722356,
            "range": "0.0509010364534673",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_identity",
            "value": 46.36542563407044,
            "range": "0.2795466792891358",
            "unit": "ns"
          },
          {
            "name": "fingerprinting/advisory_fuzzy_identity",
            "value": 62.962127670772404,
            "range": "0.14237184980258555",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/10",
            "value": 28254.745516717325,
            "range": "28.7358559157193",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/50",
            "value": 458205.8331409332,
            "range": "723.0547723757335",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/200",
            "value": 5787452.833333334,
            "range": "5683.299899100107",
            "unit": "ns"
          },
          {
            "name": "post_process_ngrams/pairwise_comparison/500",
            "value": 33206929.25,
            "range": "150185.89473366737",
            "unit": "ns"
          }
        ]
      }
    ]
  }
}