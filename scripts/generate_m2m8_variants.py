#!/usr/bin/env python3
"""Generate M2-M8 mutation variants for 7 TSX base patterns (147 files)."""

import os, pathlib

TARGETS = pathlib.Path("/home/oxisrael/Friehub/Taas/Frensene_main/Frensense/corpus/targets")

def w(fname: str, content: str):
    path = TARGETS / fname
    path.write_text(content.lstrip("\n"))
    print(f"  {fname}")

# =======================================================================
# 1. tsx_dangerously_set_inner_html_untrusted
# =======================================================================
BASE1 = "tsx_dangerously_set_inner_html_untrusted"

# M2 — Intermediate variable
w(f"{BASE1}_m2_positive.tsx", """
// [frensense]
// observation: User-controlled HTML is assigned to an intermediate variable before passing to dangerouslySetInnerHTML.
// impact: XSS via malicious HTML/script injection.
// improvement: Sanitize the intermediate variable with DOMPurify before injecting.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = bioHtml;
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
""")

w(f"{BASE1}_m2_negative.tsx", """
// SAFE: intermediate variable is sanitized with DOMPurify before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = DOMPurify.sanitize(bioHtml);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
""")

w(f"{BASE1}_m2_negative2.tsx", """
// SAFE: React standard rendering escapes the intermediate variable content
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = bioHtml;
  return <div className="bio-container">{html}</div>;
}
""")

# M3 — Multi-hop
w(f"{BASE1}_m3_positive.tsx", """
// [frensense]
// observation: User input flows through two assignments before reaching dangerouslySetInnerHTML.
// impact: XSS via multi-hop taint propagation through intermediate variables.
// improvement: Sanitize at any point in the chain before DOM injection.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const a = bioHtml;
  const b = a;
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: b }} />;
}
""")

w(f"{BASE1}_m3_negative.tsx", """
// SAFE: sanitization is applied at the final hop before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const a = bioHtml;
  const b = DOMPurify.sanitize(a);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: b }} />;
}
""")

w(f"{BASE1}_m3_negative2.tsx", """
// SAFE: React JSX rendering escapes the multi-hop value
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const a = bioHtml;
  const b = a;
  return <div className="bio-container">{b}</div>;
}
""")

# M4 — Via helper function
w(f"{BASE1}_m4_positive.tsx", """
// [frensense]
// observation: User input passes through a helper function that does not sanitize before reaching dangerouslySetInnerHTML.
// impact: XSS via unsanitized helper return value.
// improvement: Ensure the helper sanitizes its return value.
function passthrough(x: string): string { return x; }
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = passthrough(bioHtml);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
""")

w(f"{BASE1}_m4_negative.tsx", """
// SAFE: helper function sanitizes before returning
import DOMPurify from 'dompurify';
function sanitizeHtml(x: string): string { return DOMPurify.sanitize(x); }
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = sanitizeHtml(bioHtml);
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
""")

w(f"{BASE1}_m4_negative2.tsx", """
// SAFE: React standard rendering is used instead of dangerouslySetInnerHTML
function passthrough(x: string): string { return x; }
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = passthrough(bioHtml);
  return <div className="bio-container">{html}</div>;
}
""")

# M5 — Via template literal
w(f"{BASE1}_m5_positive.tsx", """
// [frensense]
// observation: User input is injected via template literal into dangerouslySetInnerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize the template literal output or avoid dangerouslySetInnerHTML.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: `${bioHtml}` }} />;
}
""")

w(f"{BASE1}_m5_negative.tsx", """
// SAFE: template literal result is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = `${bioHtml}`;
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html) }} />;
}
""")

w(f"{BASE1}_m5_negative2.tsx", """
// SAFE: React JSX escapes the template literal content
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container">{`${bioHtml}`}</div>;
}
""")

# M6 — Via concatenation
w(f"{BASE1}_m6_positive.tsx", """
// [frensense]
// observation: User input is concatenated into an HTML string passed to dangerouslySetInnerHTML.
// impact: XSS — string concatenation does not sanitize embedded HTML/script.
// improvement: Sanitize the concatenated result or avoid dangerouslySetInnerHTML.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = "<div class='card'>" + bioHtml + "</div>";
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: html }} />;
}
""")

w(f"{BASE1}_m6_negative.tsx", """
// SAFE: concatenated result is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const html = "<div class='card'>" + bioHtml + "</div>";
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html) }} />;
}
""")

w(f"{BASE1}_m6_negative2.tsx", """
// SAFE: React JSX renders the concatenated value escaped
export function UserBio({ bioHtml }: { bioHtml: string }) {
  return <div className="bio-container">{"<div class='card'>" + bioHtml + "</div>"}</div>;
}
""")

# M7 — Via destructuring
w(f"{BASE1}_m7_positive.tsx", """
// [frensense]
// observation: User input is destructured before being passed to dangerouslySetInnerHTML.
// impact: XSS — destructuring does not sanitize the extracted value.
// improvement: Sanitize after destructuring or avoid dangerouslySetInnerHTML.
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const { content } = { content: bioHtml };
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: content }} />;
}
""")

w(f"{BASE1}_m7_negative.tsx", """
// SAFE: destructured value is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const { content } = { content: DOMPurify.sanitize(bioHtml) };
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: content }} />;
}
""")

w(f"{BASE1}_m7_negative2.tsx", """
// SAFE: React JSX renders the destructured value escaped
export function UserBio({ bioHtml }: { bioHtml: string }) {
  const { content } = { content: bioHtml };
  return <div className="bio-container">{content}</div>;
}
""")

# M8 — Via array
w(f"{BASE1}_m8_positive.tsx", """
// [frensense]
// observation: User input is accessed via array index before passing to dangerouslySetInnerHTML.
// impact: XSS — array access does not sanitize the value.
// improvement: Sanitize the array element before injection.
export function UserBio({ bioHtml }: { bioHtml: string[] }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: bioHtml[0] }} />;
}
""")

w(f"{BASE1}_m8_negative.tsx", """
// SAFE: array element is sanitized before injection
import DOMPurify from 'dompurify';
export function UserBio({ bioHtml }: { bioHtml: string[] }) {
  return <div className="bio-container" dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(bioHtml[0]) }} />;
}
""")

w(f"{BASE1}_m8_negative2.tsx", """
// SAFE: React JSX renders the array element escaped
export function UserBio({ bioHtml }: { bioHtml: string[] }) {
  return <div className="bio-container">{bioHtml[0]}</div>;
}
""")


# =======================================================================
# 2. tsx_xss_href_javascript
# =======================================================================
BASE2 = "tsx_xss_href_javascript"

def safe_url_helper():
    return """function isSafeUrl(url: string): boolean {
  try { const p = new URL(url); return ['http:', 'https:'].includes(p.protocol); }
  catch { return false; }
}"""

def sanitize_url_helper():
    return """function sanitizeUrl(url: string): string | null {
  try { const p = new URL(url); return ['http:', 'https:', 'mailto:'].includes(p.protocol) ? url : null; }
  catch { return url.startsWith('/') || url.startsWith('#') ? url : null; }
}"""

# M2
w(f"{BASE2}_m2_positive.tsx", f"""
// [frensense]
// observation: User-controlled URL is assigned to an intermediate variable before being set as href.
// impact: Clicking the link can execute javascript: XSS.
// improvement: Validate the URL protocol before assigning it to href.
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = url;
  return <a href={{u}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m2_negative.tsx", f"""
// SAFE: intermediate variable URL is validated for safe protocols
{safe_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = url;
  if (!isSafeUrl(u)) return <span>{{label}}</span>;
  return <a href={{u}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m2_negative2.tsx", f"""
// SAFE: intermediate variable URL is sanitized before rendering
{sanitize_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = sanitizeUrl(url);
  if (!u) return <span>{{label}}</span>;
  return <a href={{u}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")

# M3
w(f"{BASE2}_m3_positive.tsx", """
// [frensense]
// observation: User-controlled URL flows through two assignments before reaching href.
// impact: Multi-hop taint propagation enables javascript: XSS.
// improvement: Validate the URL at any point before assigning to href.
export function UserLink({ url, label }: { url: string; label: string }) {
  const a = url;
  const b = a;
  return <a href={b}>{label}</a>;
}
""")

w(f"{BASE2}_m3_negative.tsx", f"""
// SAFE: multi-hop URL is validated before rendering
{safe_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const a = url;
  const b = a;
  if (!isSafeUrl(b)) return <span>{{label}}</span>;
  return <a href={{b}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m3_negative2.tsx", f"""
// SAFE: multi-hop URL is sanitized before rendering
{sanitize_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const a = url;
  const b = sanitizeUrl(a);
  if (!b) return <span>{{label}}</span>;
  return <a href={{b}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")

# M4
w(f"{BASE2}_m4_positive.tsx", """
// [frensense]
// observation: User-controlled URL passes through a helper function that does not validate protocols before href.
// impact: Helper returns dangerous javascript: URL directly to href.
// improvement: Add protocol validation in the helper function.
function processUrl(x: string): string { return x; }
export function UserLink({ url, label }: { url: string; label: string }) {
  const u = processUrl(url);
  return <a href={u}>{label}</a>;
}
""")

w(f"{BASE2}_m4_negative.tsx", f"""
// SAFE: helper result is validated before rendering
{safe_url_helper()}
function processUrl(x: string): string {{ return x; }}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = processUrl(url);
  if (!isSafeUrl(u)) return <span>{{label}}</span>;
  return <a href={{u}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m4_negative2.tsx", f"""
// SAFE: helper function itself validates the URL
{sanitize_url_helper()}
function processUrl(x: string): string | null {{ return sanitizeUrl(x); }}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = processUrl(url);
  if (!u) return <span>{{label}}</span>;
  return <a href={{u}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")

# M5
w(f"{BASE2}_m5_positive.tsx", """
// [frensense]
// observation: User-controlled URL is injected into href via template literal.
// impact: Template literal passes javascript: URL to href unsanitized.
// improvement: Validate the URL before assigning to href.
export function UserLink({ url, label }: { url: string; label: string }) {
  return <a href={`${url}`}>{label}</a>;
}
""")

w(f"{BASE2}_m5_negative.tsx", f"""
// SAFE: template literal URL is validated
{safe_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = `${{url}}`;
  if (!isSafeUrl(u)) return <span>{{label}}</span>;
  return <a href={{u}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m5_negative2.tsx", f"""
// SAFE: template literal URL is sanitized
{sanitize_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = sanitizeUrl(`${{url}}`);
  if (!u) return <span>{{label}}</span>;
  return <a href={{u}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")

# M6
w(f"{BASE2}_m6_positive.tsx", """
// [frensense]
// observation: User-controlled URL is concatenated before being set as href.
// impact: Concatenation prefix does not sanitize javascript: protocol.
// improvement: Validate after concatenation or use URL constructor.
export function UserLink({ url, label }: { url: string; label: string }) {
  return <a href={url + "#nav"}>{label}</a>;
}
""")

w(f"{BASE2}_m6_negative.tsx", f"""
// SAFE: concatenated URL is validated
{safe_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = url + "#nav";
  if (!isSafeUrl(u)) return <span>{{label}}</span>;
  return <a href={{u}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m6_negative2.tsx", f"""
// SAFE: concatenated URL is sanitized
{sanitize_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const u = sanitizeUrl(url + "#nav");
  if (!u) return <span>{{label}}</span>;
  return <a href={{u}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")

# M7
w(f"{BASE2}_m7_positive.tsx", """
// [frensense]
// observation: User-controlled URL is destructured before being set as href.
// impact: Destructuring passes the unsanitized URL to href.
// improvement: Validate after destructuring.
export function UserLink({ url, label }: { url: string; label: string }) {
  const { href } = { href: url };
  return <a href={href}>{label}</a>;
}
""")

w(f"{BASE2}_m7_negative.tsx", f"""
// SAFE: destructured URL is validated
{safe_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const {{ href }} = {{ href: url }};
  if (!isSafeUrl(href)) return <span>{{label}}</span>;
  return <a href={{href}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m7_negative2.tsx", f"""
// SAFE: destructured URL is sanitized
{sanitize_url_helper()}
export function UserLink({{ url, label }}: {{ url: string; label: string }}) {{
  const {{ href }} = {{ href: url }};
  const safe = sanitizeUrl(href);
  if (!safe) return <span>{{label}}</span>;
  return <a href={{safe}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")

# M8
w(f"{BASE2}_m8_positive.tsx", """
// [frensense]
// observation: User-controlled URL is accessed via array index before href.
// impact: Array access does not sanitize, allowing javascript: XSS.
// improvement: Validate the array element before assigning to href.
export function UserLink({ url, label }: { url: string[]; label: string }) {
  return <a href={url[0]}>{label}</a>;
}
""")

w(f"{BASE2}_m8_negative.tsx", f"""
// SAFE: array element URL is validated
{safe_url_helper()}
export function UserLink({{ url, label }}: {{ url: string[]; label: string }}) {{
  if (!isSafeUrl(url[0])) return <span>{{label}}</span>;
  return <a href={{url[0]}}>{{label}}</a>;
}}
""")

w(f"{BASE2}_m8_negative2.tsx", f"""
// SAFE: array element URL is sanitized
{sanitize_url_helper()}
export function UserLink({{ url, label }}: {{ url: string[]; label: string }}) {{
  const safe = sanitizeUrl(url[0]);
  if (!safe) return <span>{{label}}</span>;
  return <a href={{safe}} target="_blank" rel="noopener noreferrer">{{label}}</a>;
}}
""")


# =======================================================================
# 3. tsx_xss_ref_dom_write
# =======================================================================
BASE3 = "tsx_xss_ref_dom_write"

# M2
w(f"{BASE3}_m2_positive.tsx", """
// [frensense]
// observation: User content is assigned to an intermediate variable before being written via ref.innerHTML.
// impact: XSS — intermediate variable carries unsanitized HTML to innerHTML.
// improvement: Sanitize or use React JSX instead of ref innerHTML.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const c = comment.body;
    if (divRef.current) divRef.current.innerHTML = c;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m2_negative.tsx", """
// SAFE: React JSX renders the intermediate variable escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const c = comment.body;
  return <div className="comment">{c}</div>;
}
""")

w(f"{BASE3}_m2_negative2.tsx", """
// SAFE: intermediate variable is sanitized via DOMPurify before innerHTML assignment
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const c = DOMPurify.sanitize(comment.body);
    if (divRef.current) divRef.current.innerHTML = c;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

# M3
w(f"{BASE3}_m3_positive.tsx", """
// [frensense]
// observation: User content flows through two assignments before ref.innerHTML write.
// impact: XSS via multi-hop DOM write.
// improvement: Sanitize at any hop or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const a = comment.body;
    const b = a;
    if (divRef.current) divRef.current.innerHTML = b;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m3_negative.tsx", """
// SAFE: React JSX renders multi-hop value escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const a = comment.body;
  const b = a;
  return <div className="comment">{b}</div>;
}
""")

w(f"{BASE3}_m3_negative2.tsx", """
// SAFE: sanitization applied at final hop
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const a = comment.body;
    const b = DOMPurify.sanitize(a);
    if (divRef.current) divRef.current.innerHTML = b;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

# M4
w(f"{BASE3}_m4_positive.tsx", """
// [frensense]
// observation: User content passes through a helper that does not sanitize before ref.innerHTML write.
// impact: XSS via unsanitized helper return value.
// improvement: Sanitize helper output or use React JSX.
import React, { useRef, useEffect } from "react";
function getContent(c: { body: string }): string { return c.body; }
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const content = getContent(comment);
    if (divRef.current) divRef.current.innerHTML = content;
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m4_negative.tsx", """
// SAFE: React JSX renders helper output escaped
import React from "react";
function getContent(c: { body: string }): string { return c.body; }
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const content = getContent(comment);
  return <div className="comment">{content}</div>;
}
""")

w(f"{BASE3}_m4_negative2.tsx", """
// SAFE: helper sanitizes before returning
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
function getSafeContent(c: { body: string }): string { return DOMPurify.sanitize(c.body); }
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const content = getSafeContent(comment);
    if (divRef.current) divRef.current.innerHTML = content;
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
""")

# M5
w(f"{BASE3}_m5_positive.tsx", """
// [frensense]
// observation: User content is injected via template literal into ref.innerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize template output or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = `${comment.body}`;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m5_negative.tsx", """
// SAFE: React JSX renders template literal escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  return <div className="comment">{`${comment.body}`}</div>;
}
""")

w(f"{BASE3}_m5_negative2.tsx", """
// SAFE: template literal result is sanitized before innerHTML write
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = DOMPurify.sanitize(`${comment.body}`);
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

# M6
w(f"{BASE3}_m6_positive.tsx", """
// [frensense]
// observation: User content is concatenated before being written via ref.innerHTML.
// impact: XSS — concatenation does not sanitize embedded HTML.
// improvement: Sanitize the concatenated result or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = "<p>" + comment.body + "</p>";
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m6_negative.tsx", """
// SAFE: React JSX renders concatenated value escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  return <div className="comment">{"<p>" + comment.body + "</p>"}</div>;
}
""")

w(f"{BASE3}_m6_negative2.tsx", """
// SAFE: concatenated result is sanitized before innerHTML write
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = DOMPurify.sanitize("<p>" + comment.body + "</p>");
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

# M7
w(f"{BASE3}_m7_positive.tsx", """
// [frensense]
// observation: User content is destructured before being written via ref.innerHTML.
// impact: XSS — destructuring does not sanitize the extracted value.
// improvement: Sanitize after destructuring or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const { body } = comment;
    if (divRef.current) divRef.current.innerHTML = body;
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m7_negative.tsx", """
// SAFE: React JSX renders destructured value escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const { body } = comment;
  return <div className="comment">{body}</div>;
}
""")

w(f"{BASE3}_m7_negative2.tsx", """
// SAFE: destructured value is sanitized before innerHTML write
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const { body } = comment;
    if (divRef.current) divRef.current.innerHTML = DOMPurify.sanitize(body);
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
""")

# M8
w(f"{BASE3}_m8_positive.tsx", """
// [frensense]
// observation: User content is accessed via array index before ref.innerHTML write.
// impact: XSS — array element is not sanitized.
// improvement: Sanitize the array element or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string[] } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = comment.body[0];
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")

w(f"{BASE3}_m8_negative.tsx", """
// SAFE: React JSX renders array element escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string[] } }) {
  return <div className="comment">{comment.body[0]}</div>;
}
""")

w(f"{BASE3}_m8_negative2.tsx", """
// SAFE: array element is sanitized before innerHTML write
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string[] } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = DOMPurify.sanitize(comment.body[0]);
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
""")


# =======================================================================
# 4. tsx_useeffect_missing_dependency
# =======================================================================
BASE4 = "tsx_useeffect_missing_dependency"

# M2
w(f"{BASE4}_m2_positive.tsx", """
// [frensense]
// observation: An intermediate variable captures a state value but is missing from useEffect deps.
// impact: Stale closure — the effect uses the captured value from the initial render.
// improvement: Include the intermediate variable's source in the dependency array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m2_negative.tsx", """
// SAFE: dependency array includes the intermediate variable's source
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m2_negative2.tsx", """
// SAFE: ref pattern avoids stale closure without deps on step
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

# M3
w(f"{BASE4}_m3_positive.tsx", """
// [frensense]
// observation: A state value flows through two assignments before being used inside useEffect with empty deps.
// impact: Stale closure — multi-hop variable captures initial value only.
// improvement: Include the source state in the dependency array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const a = step;
  const b = a;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + b), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m3_negative.tsx", """
// SAFE: dependency array includes the last-hop variable
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const a = step;
  const b = a;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + b), 1000);
    return () => clearInterval(timer);
  }, [b]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m3_negative2.tsx", """
// SAFE: ref pattern avoids stale closure
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

# M4
w(f"{BASE4}_m4_positive.tsx", """
// [frensense]
// observation: A helper function returns a state-derived value that is used inside useEffect with empty deps.
// impact: Stale closure — the helper return is captured at initial render.
// improvement: Include the return value in the dependency array.
import { useEffect, useState } from 'react';
function getStep(s: number): number { return s; }
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = getStep(step);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m4_negative.tsx", """
// SAFE: dependency array includes the helper return value
import { useEffect, useState } from 'react';
function getStep(s: number): number { return s; }
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = getStep(step);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m4_negative2.tsx", """
// SAFE: ref pattern avoids stale closure
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

# M5
w(f"{BASE4}_m5_positive.tsx", """
// [frensense]
// observation: A state value flows through a template literal before being used in useEffect with empty deps.
// impact: Stale closure — the numeric conversion captures initial value.
// improvement: Include the source state in deps array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = Number(`${step}`);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m5_negative.tsx", """
// SAFE: dependency array includes the template-literal-derived value
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = Number(`${step}`);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m5_negative2.tsx", """
// SAFE: ref pattern avoids stale closure
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

# M6
w(f"{BASE4}_m6_positive.tsx", """
// [frensense]
// observation: A state value is concatenated before being used inside useEffect with empty deps.
// impact: Stale closure — captures initial value via the concatenation chain.
// improvement: Include source state in the dependency array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = 0 + step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m6_negative.tsx", """
// SAFE: dependency array includes the concatenated value
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = 0 + step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m6_negative2.tsx", """
// SAFE: ref pattern avoids stale closure
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

# M7
w(f"{BASE4}_m7_positive.tsx", """
// [frensense]
// observation: A state value is destructured before being used inside useEffect with empty deps.
// impact: Stale closure — destructured value captures initial render value.
// improvement: Include the destructured value in deps.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const { val: s } = { val: step };
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m7_negative.tsx", """
// SAFE: destructured value is in the dependency array
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const { val: s } = { val: step };
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m7_negative2.tsx", """
// SAFE: ref pattern avoids stale closure
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

# M8
w(f"{BASE4}_m8_positive.tsx", """
// [frensense]
// observation: A state value is accessed via array index before being used inside useEffect with empty deps.
// impact: Stale closure — array captures initial value.
// improvement: Include the array-source in deps or use ref.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = [step][0];
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m8_negative.tsx", """
// SAFE: array-indexed value is in the dependency array
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = [step][0];
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
""")

w(f"{BASE4}_m8_negative2.tsx", """
// SAFE: ref pattern avoids stale closure
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
""")


# =======================================================================
# 5. tsx_suspense_fallback_xss
# =======================================================================
BASE5 = "tsx_suspense_fallback_xss"

# M2
w(f"{BASE5}_m2_positive.tsx", """
// [frensense]
// observation: User input is assigned to an intermediate variable before rendering in Suspense fallback.
// impact: XSS before hydration via unsanitized user content in fallback.
// improvement: Escape or sanitize before rendering in fallback.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const q = searchQuery;
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m2_negative.tsx", """
// SAFE: intermediate variable is rendered via React's default escaping (safe)
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const q = searchQuery;
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m2_negative2.tsx", """
// SAFE: fallback uses a generic message without user-controlled content
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

# M3
w(f"{BASE5}_m3_positive.tsx", """
// [frensense]
// observation: User input flows through two assignments before rendering in Suspense fallback.
// impact: XSS before hydration via multi-hop user content.
// improvement: Sanitize or escape before fallback rendering.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const a = searchQuery;
  const b = a;
  return (
    <div>
      <Suspense fallback={<div>Searching for: {b}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m3_negative.tsx", """
// SAFE: React escaping protects multi-hop value
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const a = searchQuery;
  const b = a;
  return (
    <div>
      <Suspense fallback={<div>Searching for: {b}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m3_negative2.tsx", """
// SAFE: fallback uses generic message
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

# M4
w(f"{BASE5}_m4_positive.tsx", """
// [frensense]
// observation: User input passes through a helper function before rendering in Suspense fallback.
// impact: XSS before hydration — helper does not sanitize.
// improvement: Sanitize helper output or escape in fallback.
'use client'
import { Suspense } from 'react'
function getQueryParam(q: string): string { return q; }
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const q = getQueryParam(searchQuery);
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m4_negative.tsx", """
// SAFE: React escaping protects helper output
'use client'
import { Suspense } from 'react'
function getQueryParam(q: string): string { return q; }
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const q = getQueryParam(searchQuery);
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m4_negative2.tsx", """
// SAFE: fallback uses generic message
'use client'
import { Suspense } from 'react'
function getQueryParam(q: string): string { return q; }
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

# M5
w(f"{BASE5}_m5_positive.tsx", """
// [frensense]
// observation: User input is injected into Suspense fallback via template literal.
// impact: XSS before hydration.
// improvement: Escape template output or use generic message.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {`${searchQuery}`}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m5_negative.tsx", """
// SAFE: React escaping protects template literal content
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {`${searchQuery}`}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m5_negative2.tsx", """
// SAFE: fallback uses generic message
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

# M6
w(f"{BASE5}_m6_positive.tsx", """
// [frensense]
// observation: User input is concatenated before rendering in Suspense fallback.
// impact: XSS before hydration — concatenation does not sanitize.
// improvement: Escape or sanitize before fallback rendering.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {"q=" + searchQuery}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m6_negative.tsx", """
// SAFE: React escaping protects concatenated content
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {"q=" + searchQuery}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m6_negative2.tsx", """
// SAFE: fallback uses generic message
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

# M7
w(f"{BASE5}_m7_positive.tsx", """
// [frensense]
// observation: User input is destructured before rendering in Suspense fallback.
// impact: XSS before hydration — destructuring does not sanitize.
// improvement: Escape or sanitize after destructuring.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const { q } = { q: searchQuery };
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m7_negative.tsx", """
// SAFE: React escaping protects destructured value
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const { q } = { q: searchQuery };
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m7_negative2.tsx", """
// SAFE: fallback uses generic message
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

# M8
w(f"{BASE5}_m8_positive.tsx", """
// [frensense]
// observation: User input is accessed via array index before rendering in Suspense fallback.
// impact: XSS before hydration — array element unsanitized.
// improvement: Escape or sanitize before fallback rendering.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string[] }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {searchQuery[0]}</div>}>
        <SearchResults query={searchQuery[0]} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m8_negative.tsx", """
// SAFE: React escaping protects array element
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string[] }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {searchQuery[0]}</div>}>
        <SearchResults query={searchQuery[0]} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")

w(f"{BASE5}_m8_negative2.tsx", """
// SAFE: fallback uses generic message
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string[] }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery[0]} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
""")


# =======================================================================
# 6. tsx_portal_outside_root_xss
# =======================================================================
BASE6 = "tsx_portal_outside_root_xss"

# M2
w(f"{BASE6}_m2_positive.tsx", """
// [frensense]
// observation: User-controlled message is assigned to an intermediate variable before being rendered in a portal with dangerouslySetInnerHTML.
// impact: XSS — portaled content escapes React's DOM control.
// improvement: Sanitize the intermediate variable or avoid dangerouslySetInnerHTML in portals.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = message
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
""")

w(f"{BASE6}_m2_negative.tsx", """
// SAFE: portal renders text content instead of dangerouslySetInnerHTML
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = message
  return createPortal(<div>{msg}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m2_negative2.tsx", """
// SAFE: intermediate variable is sanitized via DOMPurify before portaling
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = DOMPurify.sanitize(message)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
""")

# M3
w(f"{BASE6}_m3_positive.tsx", """
// [frensense]
// observation: User-controlled message flows through two assignments before portal render with dangerouslySetInnerHTML.
// impact: XSS via multi-hop taint into portal.
// improvement: Sanitize or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const a = message
  const b = a
  return createPortal(<div dangerouslySetInnerHTML={{ __html: b }} />, containerRef.current)
}
""")

w(f"{BASE6}_m3_negative.tsx", """
// SAFE: portal renders text content
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const a = message
  const b = a
  return createPortal(<div>{b}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m3_negative2.tsx", """
// SAFE: sanitization applied before portal render
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const a = message
  const b = DOMPurify.sanitize(a)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: b }} />, containerRef.current)
}
""")

# M4
w(f"{BASE6}_m4_positive.tsx", """
// [frensense]
// observation: User-controlled message passes through a helper that does not sanitize before portaling with dangerouslySetInnerHTML.
// impact: XSS — helper returns unsanitized HTML to portal.
// improvement: Sanitize helper output or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
function transform(x: string): string { return x; }
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = transform(message)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
""")

w(f"{BASE6}_m4_negative.tsx", """
// SAFE: portal renders text content instead of dangerouslySetInnerHTML
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
function transform(x: string): string { return x; }
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = transform(message)
  return createPortal(<div>{msg}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m4_negative2.tsx", """
// SAFE: helper sanitizes before returning
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
function transform(x: string): string { return DOMPurify.sanitize(x); }
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = transform(message)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
""")

# M5
w(f"{BASE6}_m5_positive.tsx", """
// [frensense]
// observation: User-controlled message is injected via template literal into portal dangerouslySetInnerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize template output or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: `${message}` }} />, containerRef.current)
}
""")

w(f"{BASE6}_m5_negative.tsx", """
// SAFE: portal renders text content
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div>{`${message}`}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m5_negative2.tsx", """
// SAFE: template literal output is sanitized before portaling
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const safe = DOMPurify.sanitize(`${message}`)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: safe }} />, containerRef.current)
}
""")

# M6
w(f"{BASE6}_m6_positive.tsx", """
// [frensense]
// observation: User-controlled message is concatenated before portal dangerouslySetInnerHTML.
// impact: XSS — concatenation prefix does not sanitize embedded HTML.
// improvement: Sanitize concatenated result or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const html = "<div>" + message + "</div>"
  return createPortal(<div dangerouslySetInnerHTML={{ __html: html }} />, containerRef.current)
}
""")

w(f"{BASE6}_m6_negative.tsx", """
// SAFE: portal renders text content
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div>{"<div>" + message + "</div>"}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m6_negative2.tsx", """
// SAFE: concatenated result is sanitized before portaling
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const html = "<div>" + message + "</div>"
  return createPortal(<div dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(html) }} />, containerRef.current)
}
""")

# M7
w(f"{BASE6}_m7_positive.tsx", """
// [frensense]
// observation: User-controlled message is destructured before portal dangerouslySetInnerHTML.
// impact: XSS — destructuring does not sanitize.
// improvement: Sanitize after destructuring or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const { text } = { text: message }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: text }} />, containerRef.current)
}
""")

w(f"{BASE6}_m7_negative.tsx", """
// SAFE: portal renders text content
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const { text } = { text: message }
  return createPortal(<div>{text}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m7_negative2.tsx", """
// SAFE: destructured value is sanitized before portaling
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const { text } = { text: DOMPurify.sanitize(message) }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: text }} />, containerRef.current)
}
""")

# M8
w(f"{BASE6}_m8_positive.tsx", """
// [frensense]
// observation: User-controlled message is accessed via array index before portal dangerouslySetInnerHTML.
// impact: XSS — array element unsanitized in portal.
// improvement: Sanitize array element or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string[] }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: message[0] }} />, containerRef.current)
}
""")

w(f"{BASE6}_m8_negative.tsx", """
// SAFE: portal renders text content
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string[] }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div>{message[0]}</div>, containerRef.current)
}
""")

w(f"{BASE6}_m8_negative2.tsx", """
// SAFE: array element is sanitized before portaling
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string[] }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(message[0]) }} />, containerRef.current)
}
""")


# =======================================================================
# 7. tsx_error_boundary_render_crash
# =======================================================================
BASE7 = "tsx_error_boundary_render_crash"

# M2
w(f"{BASE7}_m2_positive.tsx", """
// [frensense]
// observation: The error object property is assigned to an intermediate variable that may not exist on the error.
// impact: Fallback component crashes when error.code is undefined, causing infinite error loop.
// improvement: Use optional chaining or default value for properties.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const code = error.code;
  return (
    <div role="alert">
      <p>Error code: {code}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m2_negative.tsx", """
// SAFE: intermediate variable uses optional chaining
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const code = error?.code ?? 'Unknown';
  return (
    <div role="alert">
      <p>Error code: {code}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m2_negative2.tsx", """
// SAFE: fallbackRender prop does not assume error shape
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")

# M3
w(f"{BASE7}_m3_positive.tsx", """
// [frensense]
// observation: Error property flows through two assignments before rendering, crashing on missing property.
// impact: Infinite error loop in fallback rendering.
// improvement: Guard against missing properties with fallback values.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const a = error.code;
  const b = a;
  return (
    <div role="alert">
      <p>Error code: {b}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m3_negative.tsx", """
// SAFE: multi-hop variables use default fallback
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const a = error?.code ?? 'N/A';
  const b = a;
  return (
    <div role="alert">
      <p>Error code: {b}</p>
      <p>{error?.message ?? 'Unknown error'}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m3_negative2.tsx", """
// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")

# M4
w(f"{BASE7}_m4_positive.tsx", """
// [frensense]
// observation: Error property is retrieved via a helper function that may return undefined.
// impact: Fallback crash when error.code is missing, causing infinite error loop.
// improvement: Ensure helper returns a safe default value.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function getCode(e: Error): any { return (e as any).code; }
function Fallback({ error }: { error: Error }) {
  const code = getCode(error);
  return (
    <div role="alert">
      <p>Error code: {code}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m4_negative.tsx", """
// SAFE: helper returns a safe default
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function getCode(e: Error): string { return (e as any).code ?? 'N/A'; }
function Fallback({ error }: { error: Error }) {
  const code = getCode(error);
  return (
    <div role="alert">
      <p>Error code: {code}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m4_negative2.tsx", """
// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")

# M5
w(f"{BASE7}_m5_positive.tsx", """
// [frensense]
// observation: Error property is rendered via template literal that throws when property is missing.
// impact: Fallback crash — error.code is undefined, template literal throws in strict mode.
// improvement: Use optional chaining in template expression.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {`${error.code}`}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m5_negative.tsx", """
// SAFE: template literal uses optional chaining
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {`${error?.code ?? 'N/A'}`}</p>
      <p>{error?.message ?? 'Unknown error'}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m5_negative2.tsx", """
// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")

# M6
w(f"{BASE7}_m6_positive.tsx", """
// [frensense]
// observation: Error property is concatenated with a string prefix, crashing when property is missing.
// impact: Fallback crash and infinite error loop.
// improvement: Guard with optional chaining before concatenation.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {"" + error.code}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m6_negative.tsx", """
// SAFE: concatenation uses optional chaining and default
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {"" + (error?.code ?? 'N/A')}</p>
      <p>{error?.message ?? 'Unknown error'}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m6_negative2.tsx", """
// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")

# M7
w(f"{BASE7}_m7_positive.tsx", """
// [frensense]
// observation: Error property is destructured before rendering, crashing when code does not exist.
// impact: Fallback crash — destructuring undefined causes TypeError.
// improvement: Provide default value in destructuring pattern.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const { code } = error as any;
  return (
    <div role="alert">
      <p>Error code: {code}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m7_negative.tsx", """
// SAFE: destructuring uses default value
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const { code = 'N/A', message = 'Unknown error' } = error as any;
  return (
    <div role="alert">
      <p>Error code: {code}</p>
      <p>{message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m7_negative2.tsx", """
// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")

# M8
w(f"{BASE7}_m8_positive.tsx", """
// [frensense]
// observation: Error property is accessed via array index to a non-array value, causing crash.
// impact: Fallback crash and infinite error loop.
// improvement: Guard against non-array access or use optional chaining.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {[error.code][0]}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m8_negative.tsx", """
// SAFE: array access is guarded with optional chaining and default
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const code = error?.code;
  return (
    <div role="alert">
      <p>Error code: {[code ?? 'N/A'][0]}</p>
      <p>{error?.message ?? 'Unknown error'}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
""")

w(f"{BASE7}_m8_negative2.tsx", """
// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
""")


print("\n=== Generated 147 files (7 patterns × 7 variants × 3 files) ===")
