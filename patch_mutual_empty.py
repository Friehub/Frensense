import re
content = open("frensense-engine/src/pattern/similarity.rs").read()

# Fix tainted_api_sim
content = content.replace("""    let tainted_api_sim = if candidate.tainted_api_calls.is_empty() && target.tainted_api_calls.is_empty() {
        1.0
    } else if candidate.tainted_api_calls.is_empty() {""", """    let tainted_api_sim = if candidate.tainted_api_calls.is_empty() && target.tainted_api_calls.is_empty() {
        0.0
    } else if candidate.tainted_api_calls.is_empty() {""")

# Fix cf_order_sim
content = content.replace("""    let cf_order_sim = if candidate.control_flow_sequence.is_empty()
        && target.control_flow_sequence.is_empty()
    {
        1.0
    } else {""", """    let cf_order_sim = if candidate.control_flow_sequence.is_empty()
        && target.control_flow_sequence.is_empty()
    {
        0.0
    } else {""")

open("frensense-engine/src/pattern/similarity.rs", "w").write(content)
