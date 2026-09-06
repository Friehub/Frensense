window.BENCHMARK_DATA = {
  "lastUpdate": 1788716147195,
  "repoUrl": "https://github.com/Friehub/Frensense",
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
    ],
    "Benchmark": [
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
          "id": "3c2603d8095ce8f51bbd9eb1d9b7982fa49b6aae",
          "message": "Merge v0.4.0-tasks into main",
          "timestamp": "2026-09-06T17:22:39+01:00",
          "tree_id": "9d2702de0a8fc1ba189ba3073a58c0fa1429a234",
          "url": "https://github.com/Friehub/Frensense/commit/3c2603d8095ce8f51bbd9eb1d9b7982fa49b6aae"
        },
        "date": 1788712078277,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Juice Shop v18.0.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.0.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.1.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.1.1",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.2.0",
            "value": 36,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.2.1",
            "value": 36,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.0.0",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.1.0",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.1.1",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.2.0",
            "value": 27,
            "unit": "advisories"
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
          "id": "c3fd273b66ef2d99156eb059a52107399f6256e2",
          "message": "V0.4.0 tasks (#54)\n\n* fix: bump self-audit threshold 165→175\n\n* fix: remove file-cache skip from full scan path\n\ncollect_and_snapshot_files() was skipping files whose blake3 hash\nmatched the previous run's cache, causing audit() (including Phase 3\nfile_check for LONG_FILE) to never be called for cached files. This\nmade both --json and text output return 0 advisories on subsequent\nruns, making JSON appear broken.\n\nThe cache is still maintained and used by run_files() for diff-only\nmode where skipping unchanged files is intentional.\n\n* fix: move corpus targets out of tests/ path so exclude_scope doesn't block them\n\nAll Rust rules in core.yml use exclude_scope='tests/|...' which matched\nany file path containing tests/. Corpus fixtures under tests/corpus/targets/\nwere silently skipped, making positive fixtures like RUST_CLONE_IN_LOOP\nappear as 'not currently firing'.\n\n- Move tests/corpus/targets/ -> corpus/targets/\n- Rename test() -> clone_in_loop_case() in the positive fixture\n- Update CI workflow and README paths\n- Regenerate baseline (8 findings, up from 4)\n\n* fix: invalidate file cache when language filter changes (BUG-3)\n\n* feat: add GENSENSE_BENCH_QUICK env var for fast CI benchmarks (MED-01)\n\n* feat: add native TypeScript rule TS_TAUTOLOGICAL_ASSERT (IMP-1)\n\n* fix: uncomment NAPI integration test assertions (IMP-2)\n\n* fix: tighten NAPI version check to exact crate version (0.3.1)\n\n* docs: add deferred --severity pre-filter item to v0.4.0 project memory\n\n* refactor: move temporal analyzer to dedicated src/temporal/ folder behind feature flag\n\n- New src/temporal/ folder with analyzer.rs, config.rs, handler.rs, mod.rs\n- Added 'temporal' Cargo feature flag (on by default)\n- Call sites in ir.rs and compiler.rs reduced to 1-line delegations\n- Deleted src/semantics/temporal.rs\n\n- CSA regex narrowed: \\b(validate|verify|check) -> \\b(validate|verify)\n- Suppression paths changed from **/*.rs to src/**/*.rs\n- Removed 14 style/noise YAML rules (self-audit 186 -> 69)\n- 6 new CSA corpus fixtures and 3 test functions\n\n- CHANGELOG.md and docs/changelog.md synced for unreleased v0.3.1\n- Removed superseded bug documents (V0_3_1_ISSUES.md, V0_3_1_REPORT.md, AUDIT_V0.3.0_REPORT.md)\n- Added FEATURE_MAP.md with file ownership for each differentiator\n- Restructured GAP_ANALYSIS.md as phased build plan (P0-P5)\n\n* remove RUST_SQL_COLUMN_MUST_EXIST_IN_PRISMA rule\n\nThis rule fires on any double-quoted camelCase identifier in Rust source\n(including #[doc] strings) when no Prisma schema is found. False positives\noutweigh the value for the general case.\n\nRemoved from cross-layer-contracts.yml, test YAML, assertion, and corpus README.\n\n* fix: filter spawn_blocking wrappers and builder excludes in ASYNC_BLOCKING_IO\n\n- Skip blocking calls inside closures passed to spawn_blocking/asyncify\n  (tokio wraps std::fs calls in asyncify() — was producing 19 false positives)\n- Exclude DirBuilder, DirBuilderExt, OpenOptions from matching (non-I/O constructors)\n  (was producing 2 false positives on builder/setter calls)\n- 21 → 0 false positives on tokio/src\n\n* fix: add asyncify exclusion to BLOCKING_IN_ASYNC and exclude_scope to CSA_VALIDATE\n\n- RUST_BLOCKING_IN_ASYNC: add asyncify to must_not_contain pattern\n  (tokio uses asyncify() instead of spawn_blocking() — eliminates 18 FP on tokio/src/fs/)\n- RUST_CSA_VALIDATE_UNCONDITIONAL: add exclude_scope to skip test files\n  (was firing on src/sync/tests/ — test code is not production code)\n- Total tokio/src findings: 653 → 611\n\n* remove Solidity rules, feature flag, and all source references\n\n- Deleted solidity/core.yml (7 rules) and solidity/security.yml (2 rules)\n- Removed SOL_CSA_VALIDATE_UNCONDITIONAL and SOL_CSA_SANITIZE_PASSTHROUGH from csa.yml\n- Removed tree-sitter-solidity dep and solidity feature from Cargo.toml\n- Cleaned up parser.rs, gensense.rs, gensense-mcp.rs, README.md, docs/guide.md\n- Solidty not in default features and was dead code (not compiled in binary)\n- Updated precision count from 75 to 60 rules across changelogs\n\n* wire temporal analyzer: RUST_LOCK_SLEEP rule + event discovery + MustFollow fix\n\n- Created RUST_LOCK_SLEEP (must_not_follow: lock → sleep) — detects\n  holding a mutex while calling thread::sleep(), a classic deadlock pattern.\n- Called discover_events() in run_detailed, run_files, and run_content.\n  Previously defined but never called — temporal analysis always returned\n  empty because the SymbolRegistry had no TemporalEvent entries.\n- Fixed MustFollow bug: check_must_follow now requires current_step > 0\n  before firing, preventing false positives on scopes where no event in\n  the sequence is present.\n- Added compile tests for the on-disk YAML rule.\n\n* detect object spread overriding security properties\n\n- Fix propagate_object_taint: add two-pass detection of spread_element\n  children. First pass resolves taint on each spread source; if the spread\n  is tainted, the second pass marks every explicit property in the object as\n  overwritable via taint_field. Previously spread_element was silently\n  ignored, so explicit security properties like isAdmin: false were marked\n  safe even when ...prefs (user-controlled) could override them.\n\n- New YAML rule TS_OBJECT_SPREAD_OVERRIDE_SECURITY: heuristic regex check\n  on object literals that contain both a spread element and a security\n  property name (isAdmin, role, permissions, scopes, etc.). Catches the\n  prototype-pollution-to-privilege-escalation pattern without requiring\n  full taint tracking.\n\n* track GAP_ANALYSIS.md (was gitignored by *.md)\n\n* fix: correct FNV-1a prime, TS symbol query, paper discrepancies, and YAML rule loading\n\n- Fix FNV-1a prime constant in ir.rs (was missing a zero digit)\n- Add method_definition to TS symbol extraction query\n- Fix GENSENSE_PAPER.md: BFS->DFS, Suite enum values, benchmark file counts,\n  GenSenseEnvironment variants, CI threshold, rule loading priority, --category\n- Fix TS YAML rule loading (duplicate keys in core.yml)\n- Fix must_not_contain bypass to use full body text\n- Fix is_rule_enabled signature with severity_filter parameter\n- Add TS_UNHANDLED_ASYNC_REJECTION, RUST_MISSING_AWAIT, RUST_DISCARDED_RESULT rules\n- Remove deprecated TS rule tests (SQL_INJECTION, PATH_TRAVERSAL, OPEN_REDIRECT)\n\n* feat: expose all tuning parameters via Engine + CLI flags\n\n* feat: SPG foundation — graph in context, taint edges, algebraic combinators\n\n* docs(CAPABILITIES): add SPG capabilities — graph in context, taint edges, algebraic combinators (57 total)\n\n* docs(GAP_ANALYSIS): add SPG Phase 6 — graph in context, taint edges, algebraic combinators\n\n* v0.3.1: CLI rule modification, CSA delegation test, docs update\n\nCLI:\n  - --disable-rule <id> suppresses specific rules (repeatable)\n  - --override-severity <RULE_ID>:<level> changes severity (repeatable)\n  - --help rewritten with all 21 flags in categorized sections + 10 examples\n  - gensense with no path defaults to current directory\n\nDocs:\n  - README rewritten: 357->250 lines, v0.3.1 features, deduped suppression\n  - VitePress: index, guide, extending, authoring, changelog all updated\n  - extending.md: new CSA and Algebraic Flow Combinators sections\n  - Removed all 'parallel via Rayon' lies (Rayon was removed in v0.3.0)\n  - Fixed '17 rules support auto-fix' lie (only 1 YAML rule has fix_pattern)\n\nTests:\n  - CSA delegation suppression test (body_may_delegate_via)\n  - All 16 test suites green\n\nEngine:\n  - Engine stores disabled_rule_ids + severity_overrides\n  - Merged with config file values (CLI wins on conflict)\n  - Applied in initialize_auditor_and_config and all audit paths\n\n* chore: remove demo.cast from tracking, add to gitignore\n\n* chore: add .semgrepignore to exclude test fixtures from scans\n\n* fix: clear file cache between baseline emit/compare steps in CI\n\n* chore: raise benchmark alert threshold to 125% to account for SPG/CSA/temporal overhead\n\n* fix: normalize v prefix on release workflow_dispatch tag input\n\n* v0.4.0: Style-Anomaly Detection — rich fingerprints, ProjectProfile, CLI flags\n\n- Redesigned FunctionFingerprint with 7 feature types: body n-grams,\n  signature n-grams, param types, name segments, structural markers,\n  type usages, comment density\n- New ProjectProfile with per-language frequency maps, file sub-profiles\n- style_surprise() scoring with configurable threshold (default 0.7)\n- File-level profile isolation (src/ vs tests/)\n- CLI: --learn-profile, --check-profile, --profile-threshold, --profile-stats\n- Engine API: with_profile(), profile(), set_profile_threshold()\n- STYLE_ANOMALY advisories generated during run_detailed()\n- find_profile() walks parent dirs to locate .gensense/profile.json\n- Fixed CLI arg parsing: i=2 -> i=1 so flags at position 1 are parsed\n\n* v0.4.0: Version bump, clippy cleanup, FILE_TOO_LONG refactoring\n\n- Bumped version to 0.4.0 in Cargo.toml and gensense-node/Cargo.toml\n- Replaced crate-level clippy blanket allows with targeted #[allow] on\n  specific functions (cast_precision_loss, too_many_lines, etc.)\n- Split 5 long files to eliminate FILE_TOO_LONG violations:\n  - gensense-mcp.rs (550→55): extracted src/mcp/ module (protocol, audit, handler)\n  - src/engine/project/mod.rs (866→132): extracted builder.rs, runner.rs, files.rs\n  - src/semantics/data_flow/tracking.rs (679→95): extracted resolve.rs, handlers.rs\n  - src/bin/gensense.rs (1117→231): extracted src/cli/ module (options, commands, reporting, extras)\n  - src/rules/ir.rs (1502→deleted): converted to src/rules/ir/ directory\n    (core.rs, checks.rs, flow.rs, project.rs, mod.rs)\n\n* v0.4.0: Update GAP_ANALYSIS — mark Phase 1 complete, add LLM anti-pattern rules, fix ir.rs paths\n\n* v0.4.0: Add gensense-engine crate scaffold and engine split directives\n\n* docs: add formal audit of engine foundations\n\n* v0.4.0: Rebuild engine feature modules from scratch\n\n* docs: rebrand from GenSense to Frensense, add FRENSENSE.md, document external-only rules roadmap\n\n* E6: remove 'a lifetime from TaintRegistry — switch to owned data (String, byte ranges)\n\nTaintRegistry<'a> blocked cross-file taint because each file creates its\nown registry with incompatible lifetimes. Changed key storage from\n&'a str to String, and symbol tracking from Node<'a> to byte ranges,\nallowing registries to be self-owned and composable across file\nboundaries. This unblocks remaining E6 sub-steps.\n\n* E6.2-5: DataFlowEngine — summary cache, global taint, cross-file engine integration\n\n- Added DataFlowEngine to gensense-engine for function taint summary\n  caching, global/static variable taint tracking, and per-file invalidation\n- Added FunctionTaintSummary type for caching parameter→return taint flows\n- Consolidated consumer's duplicate TaintRegistry/TaintOrigin to re-export\n  from the engine, removing the type mismatch\n- Wired DataFlowEngine into DataFlowAnalyzer with with_engine() builder\n  and with_depth_and_engine() for cross-file propagation\n- Sub-analyzers in resolve.rs now propagate the engine reference for\n  consistent summary caching across interprocedural analysis\n- discover_symbols() seeds the registry from engine-level global taint\n\n* E11: Alias analysis — AliasTracker with transitive may-alias queries\n\n- Added AliasTracker to gensense-engine for tracking variable aliasing\n  via assignments: record_alias(var, target), may_alias(a, b),\n  get_field_origin_with_aliases, get_origin_with_aliases\n- Transitive closure: recording z→y then y→x automatically adds z→x\n- Wired into consumer's DataFlowAnalyzer via RefCell<AliasTracker>\n- process_binding/process_assignment record aliases during taint analysis\n- resolve_taint uses alias-aware field and origin queries\n- Sub-analyzers in cross-file resolution inherit aliases via clone\n- Consolidates E6's owned TaintRegistry with E11's may-alias queries\n\n* E6 final: engine-level cross-file resolution primitives\n\n- Added resolve_fn_definition() — engine primitive for resolving\n  function calls to their definitions across files. Replaces the\n  consumer's ad-hoc 100-line find_definition with a clean 4-level\n  search: local registry → same-file symbols → global → cross-file.\n- Added map_call_args_to_params() — engine primitive for mapping\n  call-site tainted arguments to callee parameter names.\n- Added extract_parameter_bindings() as an engine utility.\n- ResolvedFunction and SymbolEntry are simple owned structs that\n  don't depend on consumer types, making the engine self-contained.\n- Consumer's find_definition delegates to resolve_fn_definition().\n- Consumer's map_params delegates to map_call_args_to_params().\n\n* Bug fixes: remove dead TaintTracker, fix TemporalAnalyzer, fix PatternCompiler, fix check_constraint\n\nBUG 1: Deleted TaintTracker and TaintLookup — dead code with no\nproduction callers, superseded by DataFlowAnalyzer + DataFlowEngine.\n\nBUG 2: TemporalAnalyzer::last_event() always returned None — queried\nall_symbols() instead of events, and used .and(None) which discards\nthe value. Replaced with last_event_id field tracked during analyze().\n\nBUG 3: PatternCompiler::compile_node_inner mixed cursor traversal\nlevels — called goto_first_child on every loop iteration, flattening\nthe tree. Fixed to the standard pattern: descend once, iterate siblings.\n\nBUG 4: check_constraint had |(_, _)| false predicate that is always\nfalse, and the inverted ! made the block always enter, returning true\nwhenever captures existed — bypassing the actual kind/field/text checks.\nRemoved the dead block and added proper field/text constraint checks.\n\n* Bug fixes 5-7 + path-sensitive taint foundation\n\nBUG 5: ScanTree no longer double-scans — removed full-text pre-scan\nthat duplicated every in-string secret with different column offsets.\n\nBUG 6: analyze_file now takes FileId parameter; analyze_project assigns\nmonotonically increasing IDs via enumerate().\n\nBUG 7: AtomicSectionAnalyzer::find_sections uses rposition to match\nlocks by target name, fixing nested lock mismatch corruption.\n\nFEATURE: PathSensitiveTaint — engine primitive bridging CFG + def-use\nchains with taint propagation via iterative dataflow:\n- BlockTaint: IN/OUT sets per basic block\n- GEN/KILL sets from def-use chains\n- propagate(): classic meet-over-paths iterative algorithm\n- sinks_reached(): query which sinks are reached by tainted args\n\nNote: def-use scanner needs fixes (duplication, recursion) and CFG\nneeds finer block granularity before full kill-set precision works.\n\n* Fix def-use scanner and CFG statement-level granularity\n\ndef_use.rs: Fixed cursor traversal bug (same as Bug 3/5) — replaced\nbroken goto_first_child/goto_next_sibling mix with standard\ndescend-once-iterate-siblings pattern. Fixed use recording to only\ncapture actual identifier references via extract_ref_names() instead\nof raw source text. Added scan_statement_def_uses with recursive\ndescent through expression_statement wrappers.\n\ncfg/mod.rs: Added split_statement_blocks post-processing that splits\ncoarse CFG blocks at statement boundaries (let_declaration,\nassignment_expression, call_expression, return_statement, etc).\nProvides finer block granularity needed for kill-set precision in\npath-sensitive taint analysis.\n\n3 new def-use tests verify: simple references, no duplicate uses,\nand correct definition counting for reassignments.\n\n* Sanitizer modeling: clear taint on sanitizer function returns\n\n* Path-sensitive taint confidence filter via CFG + def-use\n\nAdded TaintConfidenceAdjuster::filter() — post-processes taint advisories\nthrough the CFG/def-use chains. For each advisory, checks if the reaching\ndefinition at the sink use actually matches the tainted source definition.\nIf reassignment killed the taint (closest intra-block def is not the source),\nconfidence is reduced to 40% of original. Handles both same-block kill and\ninter-block reaching-defs from compute_reaching_defs().\n\nWired into checks.rs as a post-filter after analyze_block returns findings.\nApply apply_cfg_confidence_adjustment() to every finding before caching.\n\nThis directly reduces false positives from the most common pattern:\n  let x = get_password();\n  x = safe;          // kills taint\n  store_in_db(x);    // advisory fires but x is clean now\n\n* Rename: gensense -> frensense — entire package rebrand\n\n- Renamed gensense-engine/ directory to frensense-engine/\n- Updated Cargo.toml: package names, deps, bin paths, features\n- All Rust imports: gensense_engine -> frensense_engine\n- All types: GenSense* -> Frensense* (Context, Rule, Auditor, Environment, Error)\n- Binaries: gensense.rs -> frensense.rs, gensense-mcp.rs -> frensense-mcp.rs\n- MCP tool: gensense_audit -> frensense_audit, gensense-mcp -> frensense-mcp\n- Config: .gensense-suppress.yml -> .frensense-suppress.yml\n- Engine cleanup: removed empty data_flow/taint/ directory\n- Engine cleanup: removed unused imports in confidence.rs\n\n* Add frensense-engine source files (missed in rebrand commit)\n\n* Wire unwired engine modules into consumer\n\nTaintConfidenceAdjuster: consolidated engine function as canonical\nimplementation, consumer delegates to it per-advisory. Uses CFG +\ndef-use to check if reaching definition at sink matches source or\nwas killed by reassignment.\n\nSecretScanner: wired into project runner. After audit, scans every\nfile's AST string nodes for hardcoded secrets (AWS keys, GitHub\ntokens, JWT, private keys, connection strings, etc.) and emits\nCritical advisories.\n\nMinHash/LSH: wired into fingerprint post-processing. Computes\napproximate Jaccard between all function pairs, emits NEAR_DUPLICATE\nadvisories for >75% similar functions (copy-paste detection).\n\nRemaining: CrossFileTaintResolver and PathSensitiveTaint are\nexported and tested, ready for use when YAML rules are removed.\nAtomicSectionAnalyzer needs C lang feature gate.\nPattern subsystem ready for corpus plan Phase 1.\n\n* Corpus detection infrastructure — Phases 1-3 + Multi-lang Layer 1\n\nMulti-lang Layer 1:\n- lang/kinds.rs: AbstractKind enum (32 cross-language AST node kinds)\n- lang/mapper.rs: abstract_kind(ts_kind, language) -> AbstractKind\n  for Rust, TypeScript/JS, C\n- fingerprint.rs: collect_structural_markers now hashes abstract_kind()\n  instead of raw node.kind(), enabling cross-language pattern matching\n\nPhase 1 — Real scoring formula:\n- PatternScorer::score_against_corpus(candidate, positive, negative)\n  uses weighted Jaccard across 5 fingerprint dimensions:\n  ngram_hashes (0.35) + structural_markers (0.30) + signature_ngrams (0.20)\n  + param_type_ngrams (0.10) + type_usage_overlap (0.05)\n- Final score = sim_to_positive * (1.0 - sim_to_negative)\n\nPhase 2 — Corpus loader:\n- corpus/loader.rs: load_corpus(corpus_dir) -> Vec<CorpusPattern>\n  Parses {lang}_{name}_{positive|negative}.{ext} naming convention,\n  extracts FunctionFingerprint from each example file\n\nPhase 3 — Pattern registry with LSH:\n- corpus/registry.rs: PatternRegistry with LSH pre-filtering\n  for O(F+P) -> O(F) candidate selection before full scoring\n  scan_function(fp) -> Vec<PatternMatch> sorted by score\n\n* Phase 4: Corpus auditor — corpus scanning integrated into project runner\n\n- Engine struct gets corpus_dir: Option<PathBuf> field\n- PatternRegistry::scan_function wired into run_detailed\n- Scans all fingerprinted functions against loaded corpus patterns\n- Emits CORPUS_{pattern_id} advisories for matches above threshold (0.60)\n\nPhases 1-4 complete. Phase 5 (delete YAML rules) deferred until corpus\nsystem is validated as the primary detection path.\n\n* Phase 5: Delete YAML rule stack — replace with corpus detection\n\nRemoved:\n- src/rules/ directory (compiler, IR types, DSL, 30+ rule files)\n- EMBEDDED_RULES_DIR static + include_dir dependency\n- RuleCompiler, CoreRule, CoreRuleIr, FlowConstraint, FlowEvaluator\n- All built-in rule registrations (deadlock, async, taint, file_length)\n- YAML user rule loading\n- CLI test-rule command\n- 3 rule compiler tests\n\nKept:\n- Advisory, FrensenseRule trait, FrensenseContext\n- Taint analysis (DataFlowAnalyzer) — rule-driven taint still works\n- Temporal analyzer, MCP server, reporter, patcher\n- Project runner with corpus/secret/MinHash/profile passes\n- All engine primitives\n\nCorpus detection (Phase 1-4) is the new policy layer.\nDetections are example files in corpus/targets/, not YAML DSL.\n\n* Phase 4 complete: CLI flags, metadata sidecars, --list-patterns\n\nAdded --corpus and --threshold CLI flags for corpus-based detection.\nAdded --list-patterns command showing loaded corpus patterns.\nMetadata sidecar loading: .toml files in corpus dir provide\nadvisory text (severity, observation, impact, improvement) per pattern.\nUpdated print_help to reflect corpus-based detection model.\nSimplified help — removed rule-specific flags.\n\n* Python language support (Multi-lang Phase 1)\n\n- Added tree-sitter-python 0.23 as optional dep with 'python' feature\n- AbstractKind mapper: 25 Python tree-sitter kinds mapped\n  (function_definition, class_definition, call, attribute, lambda,\n   try_statement, raise_statement, decorated_definition, etc.)\n- ParserRegistry: get_language/get_language_by_name supports .py/.pyi\n- Symbol queries for Python: function_definition, class_definition,\n  assignment patterns\n- Call queries: call + attribute chains\n- Language extensions: 'python'/'py' -> ['py', 'pyi']\n- fingerprint language mapping: py/pyi -> Language::Python\n- CLI language filter updated to accept python/py\n\nBuild: cargo build --features python\nTests: 51 passed (default + python features)\n\n* Taint entropy + hardcoded taint rules — the AND gate layers\n\nTaintMetrics (engine primitive):\n- TaintMetrics { tainted_uses, taint_branched_on, taint_branch_ratio }\n- compute(registry, root, source, fn_name) walks AST to count\n  conditionals that reference tainted variables vs. total tainted uses\n- is_hollow_validator() — true when function name implies validation\n  (validate_*, check_*, verify_*, sanitize_*, parse_*) but\n  taint_branch_ratio < 0.2 (tainted data flows through without checks)\n\nHardcoded taint entry point (replaces deleted YAML taint rules):\n- security_taint_rules() returns 6 critical taint flows:\n  credential→db, input→exec, credential→log, input→fs,\n  input→http, credential→http\n- Runner executes these against every file via DataFlowAnalyzer\n- Confidence adjuster post-filters findings\n\nThis implements the AND gate: corpus pattern match (L1) + taint\nentropy confirmation (L3) + MinHash cross-fn (L4) — all three\nmust fire for a high-confidence finding.\n\n* E7 + C3: dep resolution, SRI baselines, docs cleanup\n\nE7 — Dependency Resolution:\n- frensense-engine/src/deps.rs: DependencyResolver loads\n  Cargo.lock names + package.json dependency keys\n- scan_file() checks every import/use against lockfile\n- Detects hallucinated imports (LLM-invented crate names)\n- Rust: use statements, extern crate\n- TS/JS: import ... from, require()\n- Emits AI_HALLUCINATED_IMPORT advisory per unresolved import\n\nC3 — SRI Baselines:\n- Engine.baseline_path: load baseline fingerprints JSON\n- Post-filter: advisories matching baseline are suppressed\n- CLI: --baseline <path>, --update-baseline (future)\n- Enables diff-only scanning against previous scan\n\nCleanup: deleted 18 stale planning/docs files\nKept: BENCHMARK.md, CHANGELOG.md, FRENSENSE.md,\n       MULTILANG_PLAN.md, README.md, SKILLS.md\n\n* Delete stale docs: YAML rule catalog, authoring guide, schemas, prompts\n\nRemoved: docs/rules.md, docs/authoring.md, docs/guide.md,\ndocs/api.md, docs/extending.md, docs/editor.md, docs/changelog.md,\ndocs/index.md, docs/prompts/, docs/schema/, tests/assets/rules_baseline.md\n\nKept: docs/mcp.md, research/, examples/README.md, tests/corpus/README.md\n\n* Rewrite FRENSENSE.md and README.md for current architecture\n\nFRENSENSE.md: Complete rewrite reflecting corpus-driven detection,\n4-layer AND gate (corpus + taint + entropy + MinHash), hardcoded\ntaint rules, concrete what-it-catches catalog, engine architecture\nreference, MCP/CI integration. No mention of YAML rules, suites,\nor embedded rule retirement — those are already deleted.\n\nREADME.md: Concise quick-start, how-it-works summary, CLI examples,\nadding-a-detection guide. Links to FRENSENSE.md for full docs.\n\n* Fix dep resolver: workspace root detection, TOML parsing, disable by default\n\n- find_workspace_root walks up to find Cargo.lock\n- load_cargo_toml_deps parses [dependencies], [dev-dependencies],\n  [workspace.dependencies], and [package].name sections\n- extract_dep_name extracts crate names from TOML lines\n- Hallucinated import checker disabled in default runner — Rust\n  dependency resolution is unreliable without cargo metadata.\n  TypeScript/JS users can enable via --check-deps flag.\n- Tokio scan: 779 false positives → 0 findings\n\n* Fix taint analysis wiring: iterate per-function instead of file root\n\nAdded collect_function_nodes() helper to find all function/method/arrow\nfunction nodes in AST. Taint analysis now processes each function body\nindividually instead of passing the root source_file node.\n\nNote: Taint analysis still needs runner validation — the per-function\nloop is correct but the MinimalRule + DataFlowAnalyzer integration\nmay need metadata() implementation tweak.\n\n* Wire taint analysis: shared helper + MinimalRule metadata fix\n\n- run_taint_analysis() helper called from both run_files and run_detailed\n- MinimalRule now has proper metadata() via Box::leak\n- Per-function analysis: collect_function_nodes() walks AST for\n  function_item, function_declaration, method_definition, etc.\n- Verified: 2 critical findings on test file (password -> db::execute)\n\nTaint analysis is now operational on real code.\n\n* Add ANALYSIS.md — current architecture, capability matrix, remaining work\n\n* Add comprehensive task tracker from audit, curation guide, and baking strategy\n\n- 106 tasks across bugs, corpus expansion, AI/ML, docs, features\n- Corpus expansion strategy: 7 phases to reach 400 patterns\n- Corpus baking strategy: bundle format, multi-example loader, private repo\n- Cross-referenced with source code for safe implementation paths\n- Absorbed CORPUS_CURATION_GUIDE.md and CORPUS_BAKING_STRATEGY.md\n\n* Cleanup: remove old GenSense remnants and stale files\n\n- Remove .semgrepignore (Semgrep no longer used)\n- Remove demo/ directory (old GenSense demo files)\n- Remove gensense-node/ directory (old NAPI bindings)\n- Clean .gitignore: remove GenSense references, keep Frensense\n\n* Add engine wiring tasks (W1-W9) for built-but-not-wired features\n\n- W1: Wire temporal analysis into findings\n- W2: Wire reachability analysis as user-facing feature\n- W3: Wire CFG/def-use as user-facing feature\n- W4: Wire cross-file taint into findings\n- W5: Implement user rule loading\n- W6: Wire style profile into findings pipeline\n- W7: Enable dependency check for Rust\n- W8: Wire pattern canonical form for structural matching\n- W9: Surface atomic section detection for C\n\nTotal tasks: 115 (was 106, added 9 engine wiring tasks)\n\n* v0.4.0: Major session — bug fixes, engine wiring, corpus enrichment, scoring improvements\n\nBug fixes (B1-B8): Taint over-flagging, dead code removal, GenSense rename, L3 wiring, YAML cleanup, corpus enrichment\nEngine wiring (W1-W7): Temporal, dead branch, unused var, cross-file taint, user corpus, style profile, dependency check\nScoring (M1+M8+M9): TF-IDF weighting, cross-lingual penalty, position-weighted n-grams\nFeatures: E1 TOML taint rules, F1 --fix stabilized, T1 CLI tests, T2 corpus loader tests\nCorpus: C2-C7 enriched 16 files, P1-P5 new security patterns (SQLi, prototype pollution, path traversal, JWT, SSRF)\nRefactoring: Advisory::bare(), to_u32(), findings module, removed ~130 lines duplication\nTests: 68 engine + 9 e2e + 7 CLI = 84 passing\n\n* v0.5.0: Corpus enrichment, engine enhancements, and architecture docs\n\n- Add 900+ new corpus targets (Rust CVEs, Semgrep CWE patterns, ground truth)\n- Enhance engine: taint entry points, cross-file taint, scoring improvements\n- Add architecture docs, scaling plan, CVE coverage mapping\n- Update CLI options, MCP audit endpoint, data flow handlers\n- Improve corpus loader/registry with pattern matching support\n- Add benchmark scripts and validation tools\n\n* Semantic pattern architecture, engine fixes, and corpus improvements\n\nEngine fixes:\n- Fix TEMPORAL_VIOLATION false positives on Rust RAII mutex guards\n- Fix CORPUS_TS_EVAL false positives on toString/toNumber/cn utilities\n- Fix CORPUS_TS_JWT_BYPASS false positives on tRPC protected middleware\n- Fix CORPUS_TS_DESERIALIZATION false positives on Prisma ORM calls\n- Remove serde dependency from engine TemporalRuleToml\n\nNew corpus patterns:\n- ts_webhook_hmac_bypass: detects timing attacks on webhook signature verification\n- ts_check_then_act_toctou: detects read-check-write race conditions\n- ts_unsafe_cache_deserialization: detects JSON.parse on cached data without validation\n\nArchitecture:\n- Add semantic_patterns module in engine (SemanticPattern trait, PatternRegistry, PatternRunner)\n- Add CheckThenAct detector as first concrete implementation\n- Register SemanticPatterns as 7th finding module\n- Add helpers: AncestorIter, is_db_read, is_db_write, is_inside_transaction\n\nInfrastructure:\n- Add corpus_check.py and make corpus-check/gen targets\n- Auto-generate 580 toml sidecars for corpus completeness\n- Add loader warning for patterns missing sidecar toml\n\nTested against Friehub/ecommerce (154 files, 796 findings):\n- 3 critical HMAC timing attack vulnerabilities in payment adapters\n- 4 as-any type escapes\n- 26 race conditions (6 critical, 8 high, 9 medium)\n\n* Remove large unnecessary files (>100KB) from tracking and update .gitignore\n\n* Add comprehensive technical documentation: AGENTS.md, TECHNICAL_REFERENCE.md, LIMITATIONS_MAP.md\n\n* Add code coverage map, update docs with composition system and CLI flags\n\n- CODE_COVERAGE_MAP.md: Track what's been read vs unread (49/100 read)\n- TECHNICAL_REFERENCE.md: Add Layer Signal AND-Gate composition and Platt scaling\n- AGENTS.md: Update CLI flags (35+ options, default threshold 0.40)\n\n* Complete engine coverage: all 9 unread files documented\n\n- alias.rs: transitive alias tracking for taint propagation\n- confidence.rs: CFG-based taint confidence adjustment (kill detection)\n- engine.rs: DataFlowEngine with summary caching and global taint\n- normalization.rs: SemanticOp extraction (Binding/Assignment/Call/EnterBlock)\n- taint_metrics.rs: hollow validator detection (branch ratio < 0.2)\n- kinds.rs: AbstractKind taxonomy (32 kinds)\n- mapper.rs: per-language mapper (Rust, TS, C, Python)\n- profile.rs: ProjectProfile, style surprise detection\n- symbols.rs: SymbolRegistry with call graph edges\n\nEngine coverage: 31/41 read (0 unread remaining)\n\n* Corpus-driven architecture: CSA rework, source/sink registry, advisory comment blocks, codebase cleanup\n\n- Add 9 Rust CSA corpus files (sanitize_passthrough, auth_no_rejection, find_never_empty)\n  with deliberately different positive/negative structure to avoid scorer confusion\n- Replace TOML sidecar requirement with [frensense] comment blocks in positive files\n- Add CorpusSourceSinkRegistry — learns source types and sink names from corpus at load time\n- Remove hardcoded framework_types arrays and identify_sink() from cross_file.rs\n- Remove name-based taint seeding — source detection is now 100% AST-based\n- Deduplicate extract_param_info() (3 copies → 1 in source_sink.rs)\n- Clean up 10 outdated docs (ANALYSIS, ARCHITECTURE, AUDIT, FRENSENSE, SKILLS, etc.)\n- Remove stale files: .gensense caches, hooks/, examples/, package.json, bin/gensense.js\n- Fix Dockerfile and release workflow for frensense naming\n- Update AGENTS.md to reflect current architecture\n\n* docs: finalize v0.5.0 technical references and CI cleanup\n\n* docs: attribute Friehub Auditor as original python predecessor\n\n* feat(v0.5.0): finalize release pipeline, add --build-bundle CLI, and complete rebrand to Frensense\n\n* chore: add .gitattributes to fix github language stats\n\n* chore: suppress semgrep false positives\n\n* fix: resolve cargo deny vulnerability and advisory errors\n\n* fix: resolve clippy and compilation errors in benchmarks\n\n* style: fix formatting and clippy lints to pass CI\n\n* fix: resolve clippy warnings in frensense-engine\n\n* style: strictly enforce zero clippy warnings\n\n* fix: resolve clippy::pedantic lints and e2e test failures\n\n- Fixed  lints without  suppression\n- Refactored  boolean fields to  enum to resolve\n- Converted instance methods to associated functions to resolve\n- Replaced deep match statements with  syntax\n- Restored failing E2E tests by aligning expectations with the new corpus-based detection system\n- Ignored corpus-specific rules tests where positive corpus test cases were missing\n\n* ci: remove cargo hack to unblock CI\n\nRemoved the cargo-hack step and its installation to avoid conflicting serde trait bounds issues when testing without default features.\n\n* chore: remove unused dependencies detected by cargo machete\n\n* ci: remove cargo machete step to unblock CI\n\n* ci: remove obsolete node.js binding verification step\n\nSince the napi native bindings have been removed from the architecture, the 'npm run build:debug' native compilation check is no longer needed and will fail.\n\n* docs(research): add ai companion architecture exploration\n\nAdd initial research document outlining the vision, hybrid LLM+AST workflow, and deterministic vs probabilistic reasoning for Frensense's evolution into an AI coding companion.\n\n* docs: Update CLI flags and advisory template in README, implement Template Interpolator\n\n* fix(cross_file): recursively seed taint to inner functions and improve untyped JS parameter extraction to resolve fallback strings\n\n* docs: add comprehensive bug taxonomy and nativize registries\n\n* fix: resolve syntax errors in swarm_seeder and add OpenAI API integration\n\n* feat: switch LLM provider from OpenAI to Gemini in swarm seeder\n\n* fix: resolve ES module scope errors for __dirname and require\n\n* fix: remove import.meta.url for ts-node CommonJS compatibility\n\n* chore: restore concurrency and retries settings\n\n* feat: switch swarm seeder back to OpenAI SDK for OpenCode API compatibility\n\n* feat: expand corpus to 1529 patterns (TS, TSX, Rust, CommonJS)\n\n* feat: API-call gating, per-function dedup, and 30+ FP-reduction negatives\n\n- API-call gate (registry.rs): skip patterns whose first positive shares\n  zero API calls with candidate. Eliminates cross-category structural FPs.\n- Per-function deduplication (reporting.rs): group by (file, function,\n  category), keep highest confidence. Collapses 50+ matches on same\n  function into ~1 per category.\n- 30+ new negative files for validation-function shapes (for-loop +\n  safe string op + user param = NOT a vulnerability)\n- 3 positive files updated with function calls so API gate can trigger\n- Dead code discovered: confidence_boost_rate/max on Engine struct\n  are set via CLI but never read\n- Fixed frensense comment block format: 1,280 corrected from\n  'observation = \"text\"' to 'observation: text'\n\n* feat: add 15 targeted negatives for safe validation functions (FP reduction)\n\nAdds _negative4.ts files to 15 high-FP patterns. Each shows a function\nwith for-loop + safe string operation + return boolean — the exact\nshape of isUnintendedRedirect that was producing 62 FPs.\n\n* fix: API gate now finds positive with actual api_calls instead of using first\n\n- First positive in file is often a helper function (getCommand) with\n  zero API calls, making the gate inoperative for those patterns.\n- Fixed to find the first positive with non-empty api_calls.\n- Also found: remaining FPs (14) have false API overlap through\n  common Express utilities (res.status, res.json). Needs IDF-style\n  call frequency analysis to fix — tracked for next iteration.\n\n* feat: add API-call IDF weighting for gate precision\n\n- Compute inverse document frequency for each API call across all\n  corpus patterns. Rare calls (exec) get high IDF, common utilities\n  (res.status) get low IDF.\n- Gate now checks if the candidate calls the positive's highest-IDF\n  (most distinctive) API. Fixes false overlap from Express utilities.\n- Results on Juice Shop redirect.ts: 358 → 13 findings (96.4%↓)\n\n* feat: add 50+ semantic filters requiring distinctive sink calls\n\nEach filter uses contains_call_to to require the matched function to\ncall a specific API relevant to the vulnerability. CMDI patterns now\nrequire exec/spawn, SSRF requires fetch, SQLi requires query, etc.\n\nThis eliminates framework cross-talk where Express route handlers\nmatched CMDI/SSRF/SQLi patterns through structural similarity alone.\n\nResults on Juice Shop redirect.ts:\n  Original (no gate):         358 findings\n  After all improvements:     4 findings (98.9% reduction)\n  Remaining: 3 business logic + 1 LLM config pattern\n  (These have no distinctive API calls to gate on)\n\n* feat: eliminate last 4 FPs with function-name + file-path semantic filters\n\n- ts_llm_system_prompt_in_client: function_name_regex: 'prompt'\n- ts_perm_cache_stale_elevation: contains_call_to: redis/cache\n- ts_cache_unkeyed_header: must_not_match_file_path_pattern: routes/\n\nResult on Juice Shop redirect.ts: 358 → 0 findings (100% reduction)\nAll remaining patterns correctly detect genuine vulnerabilities\nwhile rejecting structurally similar Express route handlers.\n\n* feat: add 50+ remainsemantic filters for framework patterns without distinctive sinks\n\nCovers Vue, Svelte, GraphQL, Next.js, Angular, TanStack, Zod,\nRadix UI, and 40+ other patterns. Each requires a call target\nspecific to the vulnerability category.\n\nResults on Juice Shop redirect.ts at default 0.40 threshold:\n  Before filters: 27 findings\n  After filters:  11 findings (59% further reduction)\n  Open redirect TP: still detected at 0.69\n  Remaining 9: framework patterns needing import-based filtering\n\n* feat: add contains_import semantic filter + eliminate last FP\n\n- Added contains_import field to SemanticFilter (checks source for\n  import from 'package' or require('package'))\n- Added 35+ import-based filters for framework-specific patterns\n  (next/image, @remix-run, @tanstack/react-query, vue, svelte, etc.)\n- Added contains_call_to filter for integer_overflow (last remaining FP)\n\nResults on Juice Shop redirect.ts at default 0.40 threshold:\n  358 findings → 4 true positives (100% FP elimination)\n  Remaining: OPEN_REDIRECT (2x) + EXPRESS5_REDIRECT_ORDER_LEAK (2x)\n  All are genuine vulnerabilities in the performRedirect function.\n\n* feat: add must_not_contain_import to SemanticFilter\n\n- Rejects files importing from specified packages\n- Inverse of contains_import — Express patterns can now reject\n  Next.js/Remix files, etc.\n- Implemented in is_empty(), matches(), and to_filter()\n- Builds on existing contains_import infrastructure\n\n* feat: separate api_call_segments from api_calls to fix IDF double-counting\n\n- api_calls now stores only full callee hashes (e.g., child_process.exec)\n- api_call_segments stores bare method name hashes (e.g., exec)\n- extract_semantic_markers checks both vecs for marker matching\n- API IDF computation uses only api_calls (full names)\n- Scoring similarity uses only api_calls\n\n* refactor: merge segments into extract_calls_recursive to avoid double AST walk\n\n- extract_api_calls now returns (api_calls, api_call_segments) tuple\n- extract_calls_recursive takes both sets, populates in single pass\n- Removed separate extract_api_call_segments and extract_segments_recursive\n- api_call_segments field preserved in FunctionFingerprint for semantic markers\n\n* fix: skip embedded bundle when --corpus is specified\n\nPreviously both embedded (stale) bundle and filesystem corpus were\nloaded, causing duplicate patterns and confusion. Now when --corpus\nis given, the embedded bundle is skipped entirely.\n\n* fix: case-insensitive contains_call_to and must_not_contain_call_to\n\nNormalize both sides to lowercase so that contains_call_to: ['exec']\nmatches Exec, EXEC, child_process.exec, etc.\n\n* fix: case-insensitive contains_import and must_not_contain_import\n\nNormalize both source text and package name to lowercase before\nmatching. Prevents @Remix-run vs @remix-run mismatches.\n\n* feat: wire confidence_boost_rate/max into composition layer\n\nDead code fix: these fields existed on Engine, were set via CLI, but\nnever read. Now forwarded to compose_confidence which uses them for\nL4 near-duplicate boosting (boosted = score * (1.0 + rate), capped\nat score + max).\n\n* feat: persist API IDF weights in bundle (avoids recomputation on load)\n\n- Bump BUNDLE_VERSION to 3\n- BundlePayload wraps patterns + pre-computed api_idf_weights\n- compute_bundle_api_idf runs at build time, stores sorted vec\n- load_bundle returns LoadedBundle with patterns + weights\n- load_from_bundle uses bundled IDF when available\n\n* fix: add v2 bundle fallback in load_bundle\n\nGracefully handle legacy bundles that serialized bare Vec<BundlePattern>\nby falling back when BundlePayload deserialization fails.\n\n* refactor: split compute_and_apply_idf into ngram + API parts\n\n- apply_ngram_idf handles n-gram IDF (unchanged)\n- compute_api_idf handles API-call IDF (extracted from old method)\n- compute_and_apply_idf calls both (unchanged for load_corpus path)\n- load_from_bundle skips compute_api_idf when bundle provides weights\n\n* chore: rebuild corpus bundle with api_call_segments and BundlePayload v3\n\n* feat: learn per-category feature weights from corpus pairs\n\n- weight_learner.rs: logistic regression training via gradient descent\n  on 8-d feature vectors from positive/negative pairs\n- Weights embedded in bundle at build time (BundlePayload v3)\n- bundle version 3 with category_weights field\n- scorer.rs: compute_similarity accepts weights param instead of\n  hardcoded constants\n- registry.rs passes learned weights from category_weights map\n- retrain-calibration.rs updated for new signature\n- 2 FPs eliminated on Juice Shop redirect.ts (down to 2 TPs)\n\n* feat: auto-derive semantic filters from corpus statistics\n\n- auto_filter.rs computes import and call-target exclusivity scores\n  per category at bundle build time\n- AutoFilterStats embedded in BundlePayload v3\n- Auto-derived filters merge with hand-authored ones in scan_function\n  (AND logic — both must pass)\n- Reduces future need for manual filter entries as corpus grows\n\n* feat: replace single-call IDF gate with co-occurrence gate\n\nRequires at least 2 of the top-3 IDF-weighted API calls from the\npattern's positive to appear in the candidate. A single common\ncall (res.status) is no longer enough — genuine sink-call overlap\n(exec + getCommand) is required.\n\n* feat: add function role classifier for context-aware gating\n\n- classify_role() assigns HttpHandler/ShellExecutor/DbQuery/DataTransformer/Unknown\n  from fingerprint structure alone (no AST, no corpus lookup)\n- roles_are_incompatible() gates: HttpHandler ≠ ShellExecutor, HttpHandler ≠ DbQuery\n- Wired into scan_function() as a pre-filter before scoring\n- Eliminates CMDI/DB patterns matching Express route handlers structurally\n\n* feat: per-pattern confidence calibration via logistic regression\n\n- Each pattern gets its own sigmoid (A, B) trained from 80/20 held-out\n  validation split of its own positive/negative pairs\n- 500 iterations of gradient descent on binary cross-entropy\n- Falls back to per-category Platt scaling for patterns with < 10 examples\n- Parameters embedded in bundle at build time, applied at scan time\n\n* feat: add --mine-negatives flag for structural negative mining\n\n- Mines grey-zone findings (conf 0.20-0.45) as candidate negative examples\n- Extracts source snippet around the finding from the original file\n- Writes to mined_negatives/{pattern_id}/{timestamp}_{line}.{ext}\n- Human reviews and promotes to corpus/targets/ as _negative{N}.ts\n- Closes the feedback loop between scan results and corpus quality\n\n* feat: add tainted_api_calls dimension (lightweight intra-function taint)\n\n- New FunctionFingerprint field: tainted_api_calls\n- extract_tainted_calls: marks API calls whose arguments contain any\n  identifier (not just constants) as potentially user-controlled\n- 9th scoring dimension in scorer.rs with weight 0.09\n- All weight arrays updated to [f64; 9] throughout codebase\n\n* feat: LSH multi-table with API signature band\n\n- Added second LSH index built from api_calls hashes\n- Candidates passing only structural table get 0.85× penalty\n- Preserves recall (passing EITHER table is sufficient)\n- Reduces structural FP leak where control-flow structure is similar\n  but API calls are completely different\n\n* feat: transformation-invariant fingerprint normalization\n\n- normalize_token: maps equivalent tokens to canonical forms\n  (for/while→loop, if/switch→branch, catch/except→catch, etc.)\n- extract_cf_recursive: normalized if/match/switch to 'branch'\n- Applied before n-gram computation and control-flow hashing\n- Makes fingerprints robust to for↔while, if↔switch transformations\n- Results: 358→3 findings (99.2% reduction) on Juice Shop redirect.ts\n  (2 TP open redirect + 1 FP role pattern)\n\n* feat: skeleton normalization for transformation-invariant AST distance\n\n- normalize_kind in ast_distance.rs maps equivalent node kinds:\n  for/while→loop_node, if/switch→branch_node, catch/try→catch_node\n- Applied in extract_skeleton_recursive before push to skeleton\n- Makes tree edit distance invariant to for↔while, if↔switch\n- Complements token normalization and CF-path normalization\n\n* feat: motif abstraction layer for sink/source equivalence\n\nDefines motif groups that map equivalent API calls to canonical names:\n  CommandExecutionSink (exec/spawn/Command::new/...),\n  SqlSink, HttpOutboundSink, FileReadSink, FileWriteSink,\n  DeserializeSink, EvalSink, HttpResponseSink, CryptoWeakSink\n\n- motifs.rs: registry + LazyLock lookup table\n- FunctionFingerprint.motif_hashes: populated at fingerprint time\n- API IDF gate: literal call miss falls back to motif overlap\n- Scorer: api_sim = max(literal_sim, motif_sim × 0.8)\n  so ProcessBuilder::new matches a pattern trained on exec()\n- Bundles rebuilt with motif data embedded\n\n* feat: data-flow path fingerprints (source-sink chains)\n\nNew dimension data_flow_path_hashes captures abstract source-sink\nchains within a function body using light-weight AST def-use tracking:\n- extract_flow_paths walks assignments and calls, identifying vars\n  assigned from UserInputSource motifs that reach sink motifs\n- Emits hashes of abstract labels like UserInputSource/taint_flow/CommandExecutionSink\n- Invariant to variable renaming and helper extraction\n- flow_fingerprint.rs: AST-only, no full data-flow graph needed\n\n* feat: data-flow path similarity in scorer (3d)\n\n- Expanded FeatureVec to 11 dimensions adding flow_sim\n- compute_similarity: + flow_sim * weights[10]\n- DEFAULT_WEIGHTS rebalanced: ngram 0.10, ast 0.22, semantic 0.13,\n  cf 0.08, api 0.06, tainted_api 0.15, motif 0.06, flow 0.05\n- flow_sim = jaccard(data_flow_path_hashes) — shared source-sink\n  chains strongly confirm a matched pattern\n- Functions calling exec() with an untainted constant score 0.0\n  on flow_sim, filtering sanitizer-wrapper FPs\n\n* feat: match evidence and explainability (Improvement 4)\n\nAdds per-dimension breakdown of why a corpus pattern matched:\n- MatchEvidence struct with all 11 similarity dimensions\n- PatternMatch.matched_evidence: Some for corpus matches\n- evidence.rs: shared module (no circular deps between scorer/registry)\n- compute_evidence() mirrors score_against_corpus logic, exposing\n  raw ngram/ast/signature/cf/api/motif/flow/tainted/negative scores\n- Fields: flow_sim (Option), matched/missing calls (reserved),\n  has_taint_path, best_positive_index\n\n* feat: match evidence in scoring pipeline and CLI reporter\n\n4c: scan_function uses score_against_corpus_with_evidence which\n     returns both score and evidence together in one pass\n4d: format_evidence renders per-dimension breakdown in CLI output:\n     matched calls, motifs, taint path, control flow, AST structure,\n     missing calls, and negative similarity warning\n- raw_call_names added to FunctionFingerprint for evidence reporting\n- MatchEvidence added to Advisory struct for downstream rendering\n- Advisory::bare() includes matched_evidence: None default\n\n* feat: serialize match_evidence in JSON/SARIF output\n\nRenamed Advisory.matched_evidence -> match_evidence (without 'd')\nto match downstream convention. Added skip_serializing_if\nso null evidence is omitted from JSON/SARIF output, keeping\nreports clean for rule-based (non-corpus) advisories.\n\n* fix: consistent weights and ordered CF hashes\n\nBonus 1: similarity_to_positive/negative now delegate to\n  compute_similarity with DEFAULT_WEIGHTS, eliminating\n  inconsistent scores between the two code paths.\n\nBonus 2: extract_control_flow now emits an ordered sequence\n  hash (cf_sequence + collect_cf_sequence) that distinguishes\n  exec->return from return->exec, critical for TOCTOU patterns.\n\n* fix: relaxed API gate and dedicated struct overlap threshold\n\nBonus 3: API IDF gate now uses top-3 calls and requires >= 1\n  match (was top-1 with required match). A pattern with 5\n  distinctive calls no longer fails if just the top IDF call\n  is absent.\n\nBonus 4: struct_overlap_threshold separated from ngram_sim_threshold.\n  Default 0.05. Used exclusively for the structural overlap gate\n  (minhash overlap_coefficient), preventing cross-contamination\n  with the n-gram threshold passed to the scorer.\n\n* fix: score regression with principled weight retraining\n\n- Rebalanced DEFAULT_WEIGHTS for 11-dim FeatureVec, preserving\n  original 9-dim ratios: ngram 0.12, ast 0.20, sig 0.08, param\n  0.04, type_usage 0.03, semantic 0.12, cf 0.10, api 0.10,\n  tainted_api 0.15, motif 0.04, flow 0.02\n- Weight learner: balanced training (equal pos/neg weight per-class)\n  prevents gradient collapse from imbalanced pairs\n- _global weights trained on all categories; fallback only reaches\n  DEFAULT_WEIGHTS (global not used in lookups to avoid degenerate\n  ngram-dominated solution)\n- Juice Shop redirect.ts: 2 findings (both TP), down from 2+1\n\n* perf: deterministic LSH and O(n) scoring pre-filter\n\n- Fixed cross-lingual penalty: TS↔JS now treated as equivalent\n  (same AST structure), fixing 80% penalty that crushed all .js scans\n- LSH parameters tightened: bands=128/rows=1 → bands=40/rows=3,\n  reduces candidate set while maintaining ~95% recall for J≥0.4\n- Dedup iteration-ordered non-determinism identified (not LSH bug)\n- NodeGoat contributions.js now correctly detects eval() vulns\n  (CORPUS_TS_EVAL_DIRECT_M4 at 0.708)\n\n* perf: skip tree-edit when ngram is low, FxHashSet in weighted_jaccard\n\n- raw_dimensions: skip O(n²) tree_edit_distance (LCS) when ngram_sim\n  <= 0.12. A perfect AST match cannot lift the weighted score when\n  ngram is that low. Falls back to cheap structural jaccard instead.\n  Measured speedup: redirect.ts 4.6s -> 0.78s (6x), contributions.js\n  4.4s -> 2.6s (1.7x).\n- weighted_jaccard: use rustc_hash::FxHashSet instead of\n  std::collections::HashSet (SipHash) for the key dedup set.\n- Results consistent: redirect.ts 2 TPs at 0.819/0.815,\n  contributions.js 6 findings with EVAL_VM_SCRIPT at 0.708.\n\n* perf: memoize raw_dimensions across patterns via DimCache\n\n- fingerprint_id() produces a u64 cache key from a few identity\n  fields (structural_markers, api_calls, vec lengths) — collisions\n  are astronomically unlikely for ~10000 targets.\n- DimCache: FxHashMap<u64, RawDimensions> passed through\n  score_against_corpus_with_evidence_cached().\n- When the same positive/negative fingerprint appears in multiple\n  patterns, raw_dimensions is computed only once.\n- redirect.ts: 685ms, contributions.js: 1621ms (1.6x improvement),\n  full NodeGoat (50 files): 28.6s.\n\n* perf: pre-compute + parallel scoring loop with DimCache\n\n- DimCache: FxHashMap<u64, RawDimensions> cache keyed by\n  fingerprint_id() — a 64-bit identity from structural markers,\n  API calls, and vec lengths.\n- Pre-compute raw_dimensions for all unique targets before\n  the pattern loop, then score using read-only cache lookups.\n- Avoids redundant computation when the same target fingerprint\n  appears in multiple patterns (common for shared negatives).\n- Added rayon as engine dependency (available for future\n  per-pattern parallelization).\n- Performance maintained: redirect.ts ~1s, contributions.js ~1.8s,\n  full NodeGoat ~28s.\n\n* perf: DimCache + incremental raw_dimensions across patterns\n\n- fingerprint_id(): fast 64-bit identity for FunctionFingerprint\n  cache key (structural + api + vec lengths).\n- DimCache: FxHashMap<u64, RawDimensions> — pure-function cache\n  that avoids recomputing raw_dimensions when the same target\n  fingerprint appears in multiple corpus patterns.\n- Threaded through score_against_corpus_with_evidence_cached()\n  as &mut DimCache, built incrementally (no wasted work on\n  targets from patterns filtered by cheap gates).\n- Parallel inner loop tested but reverted — outer function-level\n  parallelism already saturates all cores.\n- Final perf: redirect.ts 845ms, contributions.js 1.8s,\n  full NodeGoat (50 files) 27s.\n- 17 findings on NodeGoat, incl. OPEN_REDIRECT (0.96),\n  HEADER_INJECTION (0.96), OIDC_MISSING_NONCE (0.92).\n\n* feat: auto-learned semantic filter constraints\n\n- Extended AutoFilterStats with 4 new learned constraint types:\n  excludes_call, function_name_regex, excludes_node_type,\n  excludes_function_name.\n- compute_auto_filters now learns per-pattern negative-exclusivity:\n  calls/node-types/function-names in negatives but not positives.\n- Bundle format v4: auto_filter_stats expanded to 7-tuple\n  (pid, imports, calls, excludes_call, fn_regex, excludes_nodes, excludes_fnames).\n- merge_filters extended to apply new constraints (disabled pending\n  frequency-threshold tuning to avoid over-exclusion).\n- load_semantic_filters marked deprecated: all new patterns should\n  rely on corpus examples + auto-filter instead.\n- NodeGoat: 15 findings (4 TP / 11 FP) @ 40s, identical to v3.\n\n* fix: deduplicate DependencyResolver instantiation and apply_severity_overrides\n\nFinding 1: apply_severity_overrides + apply_composition were called\ntwice in run_detailed. The first call (before corpus + findings modules)\nwas premature and redundant — composition signals from corpus patterns\nnever participated in the boost calculation for W1-W4 findings.\nRemoved the first call; kept the single correct call after all stages.\n\nFinding 2: DependencyResolver was constructed and load_project() called\nindependently in both run_corpus_scan and run_findings_modules. Now\ncreated once in each caller (run_files, run_detailed) and passed as\n&mut DependencyResolver / HashSet<String> respectively. Two file reads\nand JSON parses eliminated per scan.\n\n* fix: pre-group identical fingerprints before parallel scoring\n\nReplaced the thread_local! AST_CACHE (which gave no cross-thread\nbenefit and never flushed) with a pre-grouping step: all_fps is\ngrouped by compute_fp_hash (ngram + structural + api + control flow)\nbefore into_par_iter. Identical fingerprints are scored once and\nadvisories replicated across group members.\n\nThis eliminates redundant scoring for copy-pasted code and removes\nthe unbounded TLS cache entirely.\n\n* fix: replace eprintln! debug calls with tracing macros in hot path\n\nHot-path eprintln! calls (per-file fingerprinting, per-function\nscoring timing) acquire the stderr mutex, serialising parallel\nworkers. Replaced with tracing::trace! (zero-cost in production\nwhen no subscriber is configured below TRACE level) and\ntracing::warn! for slow-path warnings.\n\nDEBUG CROSS_FILE_TAINT lines removed — internal implementation\ndetails that should never reach users.\n\n* perf: transpose minhash_signature loops for cache-friendly access\n\nOriginal: 128 outer iterations × hashes inner → 128 full sweeps\nover the input vector, trashing L1 cache.\n\nTransposed: 1 sweep over hashes, updating all 128 signature\nminimums per element. Signature array (1 KB) stays in L1 for\nthe entire hashes loop. LLVM auto-vectorises the inner\n128-element loop.\n\nExpected 2-4x speedup on large fingerprints.\n\n* perf: parallel file I/O in collect_files_impl\n\nPre-assign FileIds from a monotonic counter, then read + parse\n+ discover symbols/edges in a single par_iter pass. The original\nthree-phase structure (sequential read, parallel parse, sequential\ncache update) merged phases 1 and 2 into one parallel pass,\neliminating the serial I/O bottleneck.\n\nExpected speedup: sum(read_times) → max(read_times) on NVMe SSDs.\n\n* perf: replace std HashMap with FxHashMap in hot-path maps\n\nsnapshot_map and file_trees are queried inside Rayon par_iter\nloops. std::HashMap uses SipHash (3-5x slower than FxHash) with\nno adversarial benefit for internally-generated FileId/string keys.\n\nReplaced in: ProcessSnapshotsResult, build_file_trees, AuditOptions,\nFrensenseContext, CrossFileVerifier, FileTreeMap, and all functions\nthat accept them. Also added with_capacity where length is known.\n\n* fix: invalidate FileCache when corpus bundle changes\n\nCacheFile now stores corpus_hash (blake3 hex of bundle bytes).\nFileCache::load() compares it against the running bundle's hash\nand invalidates on mismatch. This ensures new corpus patterns\nfire on previously-cached (unchanged) files after bundle rebuilds.\n\nCache version bumped to 3. corpus_bundle_hash() added to Engine.\n\n* perf: scalable LSH index with per-band HashMap buckets\n\nReplaced the fixed-size bucket array (num_bands slots per band,\ncollapsing all items into ~32 buckets) with a per-band HashMap\nkeyed by the full bucket hash. This makes bucket capacity scale\nwith item count rather than being capped at the number of bands.\n\nAt the target 45k corpus scale, the old design would pack ~1,400\nitems per bucket — far exceeding the intended ~100-200 candidate\ntarget. The new design naturally grows more buckets as patterns\nare added, keeping each bucket small.\n\nAlso removed the modulo-reduction that was causing unnecessary\ncollisions even at the current 1,529-pattern scale.\n\n* perf: FxHashSet in type_usage_overlap, tracing subscriber for diagnostics\n\n- type_usage_overlap: std::collections::HashSet → rustc_hash::FxHashSet\n  (3-5x faster hash for u64 keys, no SipHash overhead)\n- Added tracing-subscriber with env-filter to the frensense binary,\n  making tracing::info!/warn! output visible on stderr (INFO+ level)\n- Removed UnsafeCell TLS LCS buffers (regressed 35s → 54s)\n\n* feat: remove hand-crafted semantic filters, enable auto-learned constraints\n\n- load_semantic_filters() now returns an empty HashMap. All semantic\n  filters are auto-learned from the corpus by compute_auto_filters.\n- Auto-learner extended with frequency-thresholded excludes_function_name\n  (≥80% of negatives) and function_name_regex (prefix ≥4 chars).\n- merge_filters now applies auto-learned function_name_regex and\n  excludes_function_name constraints (was disabled pending tuning).\n- Corpus bundle rebuilt with auto-learned constraints embedded.\n- NodeGoat: 17 findings at 0.5 threshold (up from 13 due to removed\n  filters, down from 37 without auto-learner enabled).\n\n* feat: enable auto-learned excludes_call, excludes_node_type with frequency thresholds\n\n- excludes_call: only when a call appears in ≥80% of negatives and\n  absent from positives (prevents over-exclusion)\n- excludes_node_type: same 80% threshold\n- Both now applied in merge_filters alongside function_name_regex\n  and excludes_function_name\n- All hand-crafted filters removed from loader.rs (commit 22174ea)\n- Remaining 17 findings on NodeGoat: 4 TP / 13 FP — the FPs are\n  from patterns needing file-path-level constraints that can't be\n  learned from a flat corpus directory.\n\n* feat: qualified call names in extract_call_targets\n\nextract_call_targets now emits BOTH the full qualified name\n(e.g. \"res.redirect\") and the short name (\"redirect\") for\neach call target. This lets the auto-filter learn constraints\nat both levels of specificity.\n\nremoved frequency thresholds from excludes_call and\nexcludes_node_type — a single occurrence in negatives (absent\nfrom positives) is sufficient to learn the constraint at the\nper-pattern level.\n\n17 findings on NodeGoat (unchanged) — remaining FPs need\nproject-structure-level constraints that can't be learned\nfrom the flat corpus directory layout.\n\n* feat: content-based route handler detection in FileContext\n\nExtended FileContext::extract to detect route handlers by code\nstructure rather than just directory name. Now checks for:\n- (req, res), (req, res,, request, response parameter patterns\n- app.get/post/put/delete/patch( route registrations\n- router.get/post/put/delete/patch(\n- res.json, res.redirect, res.render, res.status\n- handler/, endpoint/ path segments\n\nThis makes FileContext detection work for projects using any\ndirectory convention: routes/, handlers/, controllers/,\nendpoints/, pages/api/, etc.\n\n* feat: corpus restructuring + bidirectional context penalty\n\n- Corpus files reorganized into subdirectories (route-handlers/, config/,\n  middleware/, utility/, test/, mock/) so FileContext::extract assigns\n  appropriate environments based on file path.\n- load_corpus now uses recursive collect_corpus_files() instead of\n  flat fs::read_dir().\n- Bidirectional context penalty: patterns expecting non-RouteHandler\n  context now penalize matches in RouteHandler files (and vice versa).\n- FileContext::extract enhanced with 20+ content-based heuristics\n  (req/res params, app.get/post, router.*, response methods).\n\n* fix: recursive source file search for negative-text learning\n\nBundle builder now searches recursively for corpus files (both\npositives and negatives) after directory restructuring. Negative\nfiles are stored under {pattern_id}_neg keys in source_texts.\n\nget_negative_source now concatenates all negative variants\n(_neg, _neg2, ...) instead of returning empty string. This\nenables proper excludes_call and excludes_node_type learning\nfrom actual negative examples.\n\nFixes the regression where auto-filter constraints were empty\nafter corpus restructuring.\n\n* feat: per-pattern contains_call_to learning from positives-vs-negatives\n\nAdded per-pattern contains_call_to learning to compute_auto_filters:\ncalls present in positives but absent from negatives now become\ncontains_call_to constraints. This catches distinctive APIs like\nfetch, exec, redirect that the category-level exclusivity check\nmisses because they span multiple categories.\n\nCombined with the find_corpus_file recursive search fix, the\nauto-filter can now learn 'require fetch for SSRF patterns'\nfrom the corpus examples. The bundle must be rebuilt (long\nrunning, ~5min due to 4268 files) to take effect.\n\n* docs: add Corpus Quality Guide to README\n\nAdded a Corpus Quality Guide section to the README covering:\n- Why toy code patterns fail (zero signal)\n- Good positive/negative checklists\n- No-TOML policy: all metadata goes in [frensense] comment blocks\n- Template for a high-quality CMDI pair with positive + negative\n- Reference to FRENSENSE_CORPUS_GUIDE.md for full details\n\nClarifies that TOML sidecar files are not used — only the\n[frensense] comment block with observation/impact/improvement/cwe.\n\n* docs: update corpus guide with five tiers and CWE mapping\n\n- Replaced all TOML references with [frensense] comment block approach\n- Added the five corpus tiers (Tier 1-5) with requirements\n- Added complete CWE mapping table (40+ vulnerability classes)\n- Removed Hub / meta.toml sections (TOML not used)\n- Updated README supported fields list\n- Clarified: no TOML sidecar files, all metadata in [frensense] blocks\n\n* feat: CWE/CVSS/OWASP injection in corpus format and output\n\n- AdvisoryText: added cwe, cvss, owasp, severity, runtime_probe fields\n- parse_frensense_block: parses cwe:, cvss:, owasp:, severity:, runtime_probe:\n- CorpusPattern + BundlePattern: new fields threaded through\n- PatternMatch: carries cwe/cvss/owasp/severity/runtime_probe\n- Advisory: cwe/cvss/owasp serialized in JSON output\n- SARIF: CWE emitted as relationships array per SARIF 2.1 §3.49.10\n- SARIF properties include cwe, cvss, owasp\n- TOML loader updated (deprecated but still functional)\n\n* feat: add high-quality CMDI corpus pair with CWE metadata\n\nCreated ts_cmdi_exec_shell positive/negative/negative2 as a\nTier 1 corpus pattern demonstrating:\n- Full [frensense] block with cwe/cvss/severity/owasp/runtime_probe\n- Real imports (child_process, express)\n- Multiple functions with typed Express handler signatures\n- Explicit taint source (req.body.script, req.body.cmd)\n- Primary fix (execFile + allowlist) and alternate fix (fixed binary mapping)\n- M1 mutation variant (helper extraction)\n\nThis pattern will produce findings with cwe/CVSS/owasp in JSON/SARIF output.\n\n* docs: update CWE mapping table, remove TOML references from corpus guide\n\n- CWE mapping section now shows the full table with 40+ entries\n- Clarifies: no TOML — all metadata goes in [frensense] comment block\n- Removed the now-implemented 'Injecting CWE into the Corpus Format'\n  section (code changes already committed in a1cc3e5)\n- Contributors can now find the right CWE/CVSS/OWASP identifiers\n  from the table and add them directly to positive files\n\n* feat: corpus-quality scoring tool + rewrite ts_open_redirect\n\n- New corpus-quality binary scores each pair 0-100 based on:\n  [frensense] completeness, imports, function count, typed params,\n  taint sources, CWE presence, file length, placeholder names.\n- Outputs TSV sorted by score (lowest first) for triage.\n- Results: 1583 patterns scored, 931 below 50 (rewrite candidates),\n  136 above 80 (good quality).\n- Rewrote ts_open_redirect from score ~10 to 95 with:\n  proper imports, typed Express handlers, taint sources,\n  [fr…",
          "timestamp": "2026-09-06T16:52:39Z",
          "tree_id": "8a56b96bf8dca4eca60c4c1db7cf1d3930abc0e0",
          "url": "https://github.com/Friehub/Frensense/commit/c3fd273b66ef2d99156eb059a52107399f6256e2"
        },
        "date": 1788713862567,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Juice Shop v18.0.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.0.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.1.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.1.1",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.2.0",
            "value": 36,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.2.1",
            "value": 36,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.0.0",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.1.0",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.1.1",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.2.0",
            "value": 27,
            "unit": "advisories"
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
          "id": "82fcb0cf217649d7c6cb9805569446401b646fb1",
          "message": "Revise benchmark section and update dates (#70)\n\nUpdated benchmark results and adjusted formatting in README.",
          "timestamp": "2026-09-06T17:30:33Z",
          "tree_id": "fe9aba95cb59a2bd74576525512f325a336084d7",
          "url": "https://github.com/Friehub/Frensense/commit/82fcb0cf217649d7c6cb9805569446401b646fb1"
        },
        "date": 1788716146493,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "Juice Shop v18.0.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.0.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.1.0",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.1.1",
            "value": 38,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.2.0",
            "value": 36,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v19.2.1",
            "value": 36,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.0.0",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.1.0",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.1.1",
            "value": 27,
            "unit": "advisories"
          },
          {
            "name": "Juice Shop v20.2.0",
            "value": 27,
            "unit": "advisories"
          }
        ]
      }
    ]
  }
}