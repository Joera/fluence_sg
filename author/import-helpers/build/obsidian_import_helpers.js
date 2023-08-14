/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./src/index.ts":
/*!**********************!*\
  !*** ./src/index.ts ***!
  \**********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var turndown__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! turndown */ \"./node_modules/turndown/lib/turndown.browser.es.js\");\n\nvar hb_clean_html = function (s) {\n    var turndownService = new turndown__WEBPACK_IMPORTED_MODULE_0__[\"default\"]();\n    var md = turndownService.turndown(s);\n    return md;\n};\n handlebars.registerHelper('clean_html', hb_clean_html);\n\n\n//# sourceURL=webpack://import-helpers/./src/index.ts?");

/***/ }),

/***/ "./node_modules/turndown/lib/turndown.browser.es.js":
/*!**********************************************************!*\
  !*** ./node_modules/turndown/lib/turndown.browser.es.js ***!
  \**********************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"default\": () => (__WEBPACK_DEFAULT_EXPORT__)\n/* harmony export */ });\nfunction extend (destination) {\n  for (var i = 1; i < arguments.length; i++) {\n    var source = arguments[i];\n    for (var key in source) {\n      if (source.hasOwnProperty(key)) destination[key] = source[key];\n    }\n  }\n  return destination\n}\n\nfunction repeat (character, count) {\n  return Array(count + 1).join(character)\n}\n\nfunction trimLeadingNewlines (string) {\n  return string.replace(/^\\n*/, '')\n}\n\nfunction trimTrailingNewlines (string) {\n  // avoid match-at-end regexp bottleneck, see #370\n  var indexEnd = string.length;\n  while (indexEnd > 0 && string[indexEnd - 1] === '\\n') indexEnd--;\n  return string.substring(0, indexEnd)\n}\n\nvar blockElements = [\n  'ADDRESS', 'ARTICLE', 'ASIDE', 'AUDIO', 'BLOCKQUOTE', 'BODY', 'CANVAS',\n  'CENTER', 'DD', 'DIR', 'DIV', 'DL', 'DT', 'FIELDSET', 'FIGCAPTION', 'FIGURE',\n  'FOOTER', 'FORM', 'FRAMESET', 'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 'HEADER',\n  'HGROUP', 'HR', 'HTML', 'ISINDEX', 'LI', 'MAIN', 'MENU', 'NAV', 'NOFRAMES',\n  'NOSCRIPT', 'OL', 'OUTPUT', 'P', 'PRE', 'SECTION', 'TABLE', 'TBODY', 'TD',\n  'TFOOT', 'TH', 'THEAD', 'TR', 'UL'\n];\n\nfunction isBlock (node) {\n  return is(node, blockElements)\n}\n\nvar voidElements = [\n  'AREA', 'BASE', 'BR', 'COL', 'COMMAND', 'EMBED', 'HR', 'IMG', 'INPUT',\n  'KEYGEN', 'LINK', 'META', 'PARAM', 'SOURCE', 'TRACK', 'WBR'\n];\n\nfunction isVoid (node) {\n  return is(node, voidElements)\n}\n\nfunction hasVoid (node) {\n  return has(node, voidElements)\n}\n\nvar meaningfulWhenBlankElements = [\n  'A', 'TABLE', 'THEAD', 'TBODY', 'TFOOT', 'TH', 'TD', 'IFRAME', 'SCRIPT',\n  'AUDIO', 'VIDEO'\n];\n\nfunction isMeaningfulWhenBlank (node) {\n  return is(node, meaningfulWhenBlankElements)\n}\n\nfunction hasMeaningfulWhenBlank (node) {\n  return has(node, meaningfulWhenBlankElements)\n}\n\nfunction is (node, tagNames) {\n  return tagNames.indexOf(node.nodeName) >= 0\n}\n\nfunction has (node, tagNames) {\n  return (\n    node.getElementsByTagName &&\n    tagNames.some(function (tagName) {\n      return node.getElementsByTagName(tagName).length\n    })\n  )\n}\n\nvar rules = {};\n\nrules.paragraph = {\n  filter: 'p',\n\n  replacement: function (content) {\n    return '\\n\\n' + content + '\\n\\n'\n  }\n};\n\nrules.lineBreak = {\n  filter: 'br',\n\n  replacement: function (content, node, options) {\n    return options.br + '\\n'\n  }\n};\n\nrules.heading = {\n  filter: ['h1', 'h2', 'h3', 'h4', 'h5', 'h6'],\n\n  replacement: function (content, node, options) {\n    var hLevel = Number(node.nodeName.charAt(1));\n\n    if (options.headingStyle === 'setext' && hLevel < 3) {\n      var underline = repeat((hLevel === 1 ? '=' : '-'), content.length);\n      return (\n        '\\n\\n' + content + '\\n' + underline + '\\n\\n'\n      )\n    } else {\n      return '\\n\\n' + repeat('#', hLevel) + ' ' + content + '\\n\\n'\n    }\n  }\n};\n\nrules.blockquote = {\n  filter: 'blockquote',\n\n  replacement: function (content) {\n    content = content.replace(/^\\n+|\\n+$/g, '');\n    content = content.replace(/^/gm, '> ');\n    return '\\n\\n' + content + '\\n\\n'\n  }\n};\n\nrules.list = {\n  filter: ['ul', 'ol'],\n\n  replacement: function (content, node) {\n    var parent = node.parentNode;\n    if (parent.nodeName === 'LI' && parent.lastElementChild === node) {\n      return '\\n' + content\n    } else {\n      return '\\n\\n' + content + '\\n\\n'\n    }\n  }\n};\n\nrules.listItem = {\n  filter: 'li',\n\n  replacement: function (content, node, options) {\n    content = content\n      .replace(/^\\n+/, '') // remove leading newlines\n      .replace(/\\n+$/, '\\n') // replace trailing newlines with just a single one\n      .replace(/\\n/gm, '\\n    '); // indent\n    var prefix = options.bulletListMarker + '   ';\n    var parent = node.parentNode;\n    if (parent.nodeName === 'OL') {\n      var start = parent.getAttribute('start');\n      var index = Array.prototype.indexOf.call(parent.children, node);\n      prefix = (start ? Number(start) + index : index + 1) + '.  ';\n    }\n    return (\n      prefix + content + (node.nextSibling && !/\\n$/.test(content) ? '\\n' : '')\n    )\n  }\n};\n\nrules.indentedCodeBlock = {\n  filter: function (node, options) {\n    return (\n      options.codeBlockStyle === 'indented' &&\n      node.nodeName === 'PRE' &&\n      node.firstChild &&\n      node.firstChild.nodeName === 'CODE'\n    )\n  },\n\n  replacement: function (content, node, options) {\n    return (\n      '\\n\\n    ' +\n      node.firstChild.textContent.replace(/\\n/g, '\\n    ') +\n      '\\n\\n'\n    )\n  }\n};\n\nrules.fencedCodeBlock = {\n  filter: function (node, options) {\n    return (\n      options.codeBlockStyle === 'fenced' &&\n      node.nodeName === 'PRE' &&\n      node.firstChild &&\n      node.firstChild.nodeName === 'CODE'\n    )\n  },\n\n  replacement: function (content, node, options) {\n    var className = node.firstChild.getAttribute('class') || '';\n    var language = (className.match(/language-(\\S+)/) || [null, ''])[1];\n    var code = node.firstChild.textContent;\n\n    var fenceChar = options.fence.charAt(0);\n    var fenceSize = 3;\n    var fenceInCodeRegex = new RegExp('^' + fenceChar + '{3,}', 'gm');\n\n    var match;\n    while ((match = fenceInCodeRegex.exec(code))) {\n      if (match[0].length >= fenceSize) {\n        fenceSize = match[0].length + 1;\n      }\n    }\n\n    var fence = repeat(fenceChar, fenceSize);\n\n    return (\n      '\\n\\n' + fence + language + '\\n' +\n      code.replace(/\\n$/, '') +\n      '\\n' + fence + '\\n\\n'\n    )\n  }\n};\n\nrules.horizontalRule = {\n  filter: 'hr',\n\n  replacement: function (content, node, options) {\n    return '\\n\\n' + options.hr + '\\n\\n'\n  }\n};\n\nrules.inlineLink = {\n  filter: function (node, options) {\n    return (\n      options.linkStyle === 'inlined' &&\n      node.nodeName === 'A' &&\n      node.getAttribute('href')\n    )\n  },\n\n  replacement: function (content, node) {\n    var href = node.getAttribute('href');\n    var title = cleanAttribute(node.getAttribute('title'));\n    if (title) title = ' \"' + title + '\"';\n    return '[' + content + '](' + href + title + ')'\n  }\n};\n\nrules.referenceLink = {\n  filter: function (node, options) {\n    return (\n      options.linkStyle === 'referenced' &&\n      node.nodeName === 'A' &&\n      node.getAttribute('href')\n    )\n  },\n\n  replacement: function (content, node, options) {\n    var href = node.getAttribute('href');\n    var title = cleanAttribute(node.getAttribute('title'));\n    if (title) title = ' \"' + title + '\"';\n    var replacement;\n    var reference;\n\n    switch (options.linkReferenceStyle) {\n      case 'collapsed':\n        replacement = '[' + content + '][]';\n        reference = '[' + content + ']: ' + href + title;\n        break\n      case 'shortcut':\n        replacement = '[' + content + ']';\n        reference = '[' + content + ']: ' + href + title;\n        break\n      default:\n        var id = this.references.length + 1;\n        replacement = '[' + content + '][' + id + ']';\n        reference = '[' + id + ']: ' + href + title;\n    }\n\n    this.references.push(reference);\n    return replacement\n  },\n\n  references: [],\n\n  append: function (options) {\n    var references = '';\n    if (this.references.length) {\n      references = '\\n\\n' + this.references.join('\\n') + '\\n\\n';\n      this.references = []; // Reset references\n    }\n    return references\n  }\n};\n\nrules.emphasis = {\n  filter: ['em', 'i'],\n\n  replacement: function (content, node, options) {\n    if (!content.trim()) return ''\n    return options.emDelimiter + content + options.emDelimiter\n  }\n};\n\nrules.strong = {\n  filter: ['strong', 'b'],\n\n  replacement: function (content, node, options) {\n    if (!content.trim()) return ''\n    return options.strongDelimiter + content + options.strongDelimiter\n  }\n};\n\nrules.code = {\n  filter: function (node) {\n    var hasSiblings = node.previousSibling || node.nextSibling;\n    var isCodeBlock = node.parentNode.nodeName === 'PRE' && !hasSiblings;\n\n    return node.nodeName === 'CODE' && !isCodeBlock\n  },\n\n  replacement: function (content) {\n    if (!content) return ''\n    content = content.replace(/\\r?\\n|\\r/g, ' ');\n\n    var extraSpace = /^`|^ .*?[^ ].* $|`$/.test(content) ? ' ' : '';\n    var delimiter = '`';\n    var matches = content.match(/`+/gm) || [];\n    while (matches.indexOf(delimiter) !== -1) delimiter = delimiter + '`';\n\n    return delimiter + extraSpace + content + extraSpace + delimiter\n  }\n};\n\nrules.image = {\n  filter: 'img',\n\n  replacement: function (content, node) {\n    var alt = cleanAttribute(node.getAttribute('alt'));\n    var src = node.getAttribute('src') || '';\n    var title = cleanAttribute(node.getAttribute('title'));\n    var titlePart = title ? ' \"' + title + '\"' : '';\n    return src ? '![' + alt + ']' + '(' + src + titlePart + ')' : ''\n  }\n};\n\nfunction cleanAttribute (attribute) {\n  return attribute ? attribute.replace(/(\\n+\\s*)+/g, '\\n') : ''\n}\n\n/**\n * Manages a collection of rules used to convert HTML to Markdown\n */\n\nfunction Rules (options) {\n  this.options = options;\n  this._keep = [];\n  this._remove = [];\n\n  this.blankRule = {\n    replacement: options.blankReplacement\n  };\n\n  this.keepReplacement = options.keepReplacement;\n\n  this.defaultRule = {\n    replacement: options.defaultReplacement\n  };\n\n  this.array = [];\n  for (var key in options.rules) this.array.push(options.rules[key]);\n}\n\nRules.prototype = {\n  add: function (key, rule) {\n    this.array.unshift(rule);\n  },\n\n  keep: function (filter) {\n    this._keep.unshift({\n      filter: filter,\n      replacement: this.keepReplacement\n    });\n  },\n\n  remove: function (filter) {\n    this._remove.unshift({\n      filter: filter,\n      replacement: function () {\n        return ''\n      }\n    });\n  },\n\n  forNode: function (node) {\n    if (node.isBlank) return this.blankRule\n    var rule;\n\n    if ((rule = findRule(this.array, node, this.options))) return rule\n    if ((rule = findRule(this._keep, node, this.options))) return rule\n    if ((rule = findRule(this._remove, node, this.options))) return rule\n\n    return this.defaultRule\n  },\n\n  forEach: function (fn) {\n    for (var i = 0; i < this.array.length; i++) fn(this.array[i], i);\n  }\n};\n\nfunction findRule (rules, node, options) {\n  for (var i = 0; i < rules.length; i++) {\n    var rule = rules[i];\n    if (filterValue(rule, node, options)) return rule\n  }\n  return void 0\n}\n\nfunction filterValue (rule, node, options) {\n  var filter = rule.filter;\n  if (typeof filter === 'string') {\n    if (filter === node.nodeName.toLowerCase()) return true\n  } else if (Array.isArray(filter)) {\n    if (filter.indexOf(node.nodeName.toLowerCase()) > -1) return true\n  } else if (typeof filter === 'function') {\n    if (filter.call(rule, node, options)) return true\n  } else {\n    throw new TypeError('`filter` needs to be a string, array, or function')\n  }\n}\n\n/**\n * The collapseWhitespace function is adapted from collapse-whitespace\n * by Luc Thevenard.\n *\n * The MIT License (MIT)\n *\n * Copyright (c) 2014 Luc Thevenard <lucthevenard@gmail.com>\n *\n * Permission is hereby granted, free of charge, to any person obtaining a copy\n * of this software and associated documentation files (the \"Software\"), to deal\n * in the Software without restriction, including without limitation the rights\n * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n * copies of the Software, and to permit persons to whom the Software is\n * furnished to do so, subject to the following conditions:\n *\n * The above copyright notice and this permission notice shall be included in\n * all copies or substantial portions of the Software.\n *\n * THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN\n * THE SOFTWARE.\n */\n\n/**\n * collapseWhitespace(options) removes extraneous whitespace from an the given element.\n *\n * @param {Object} options\n */\nfunction collapseWhitespace (options) {\n  var element = options.element;\n  var isBlock = options.isBlock;\n  var isVoid = options.isVoid;\n  var isPre = options.isPre || function (node) {\n    return node.nodeName === 'PRE'\n  };\n\n  if (!element.firstChild || isPre(element)) return\n\n  var prevText = null;\n  var keepLeadingWs = false;\n\n  var prev = null;\n  var node = next(prev, element, isPre);\n\n  while (node !== element) {\n    if (node.nodeType === 3 || node.nodeType === 4) { // Node.TEXT_NODE or Node.CDATA_SECTION_NODE\n      var text = node.data.replace(/[ \\r\\n\\t]+/g, ' ');\n\n      if ((!prevText || / $/.test(prevText.data)) &&\n          !keepLeadingWs && text[0] === ' ') {\n        text = text.substr(1);\n      }\n\n      // `text` might be empty at this point.\n      if (!text) {\n        node = remove(node);\n        continue\n      }\n\n      node.data = text;\n\n      prevText = node;\n    } else if (node.nodeType === 1) { // Node.ELEMENT_NODE\n      if (isBlock(node) || node.nodeName === 'BR') {\n        if (prevText) {\n          prevText.data = prevText.data.replace(/ $/, '');\n        }\n\n        prevText = null;\n        keepLeadingWs = false;\n      } else if (isVoid(node) || isPre(node)) {\n        // Avoid trimming space around non-block, non-BR void elements and inline PRE.\n        prevText = null;\n        keepLeadingWs = true;\n      } else if (prevText) {\n        // Drop protection if set previously.\n        keepLeadingWs = false;\n      }\n    } else {\n      node = remove(node);\n      continue\n    }\n\n    var nextNode = next(prev, node, isPre);\n    prev = node;\n    node = nextNode;\n  }\n\n  if (prevText) {\n    prevText.data = prevText.data.replace(/ $/, '');\n    if (!prevText.data) {\n      remove(prevText);\n    }\n  }\n}\n\n/**\n * remove(node) removes the given node from the DOM and returns the\n * next node in the sequence.\n *\n * @param {Node} node\n * @return {Node} node\n */\nfunction remove (node) {\n  var next = node.nextSibling || node.parentNode;\n\n  node.parentNode.removeChild(node);\n\n  return next\n}\n\n/**\n * next(prev, current, isPre) returns the next node in the sequence, given the\n * current and previous nodes.\n *\n * @param {Node} prev\n * @param {Node} current\n * @param {Function} isPre\n * @return {Node}\n */\nfunction next (prev, current, isPre) {\n  if ((prev && prev.parentNode === current) || isPre(current)) {\n    return current.nextSibling || current.parentNode\n  }\n\n  return current.firstChild || current.nextSibling || current.parentNode\n}\n\n/*\n * Set up window for Node.js\n */\n\nvar root = (typeof window !== 'undefined' ? window : {});\n\n/*\n * Parsing HTML strings\n */\n\nfunction canParseHTMLNatively () {\n  var Parser = root.DOMParser;\n  var canParse = false;\n\n  // Adapted from https://gist.github.com/1129031\n  // Firefox/Opera/IE throw errors on unsupported types\n  try {\n    // WebKit returns null on unsupported types\n    if (new Parser().parseFromString('', 'text/html')) {\n      canParse = true;\n    }\n  } catch (e) {}\n\n  return canParse\n}\n\nfunction createHTMLParser () {\n  var Parser = function () {};\n\n  {\n    if (shouldUseActiveX()) {\n      Parser.prototype.parseFromString = function (string) {\n        var doc = new window.ActiveXObject('htmlfile');\n        doc.designMode = 'on'; // disable on-page scripts\n        doc.open();\n        doc.write(string);\n        doc.close();\n        return doc\n      };\n    } else {\n      Parser.prototype.parseFromString = function (string) {\n        var doc = document.implementation.createHTMLDocument('');\n        doc.open();\n        doc.write(string);\n        doc.close();\n        return doc\n      };\n    }\n  }\n  return Parser\n}\n\nfunction shouldUseActiveX () {\n  var useActiveX = false;\n  try {\n    document.implementation.createHTMLDocument('').open();\n  } catch (e) {\n    if (window.ActiveXObject) useActiveX = true;\n  }\n  return useActiveX\n}\n\nvar HTMLParser = canParseHTMLNatively() ? root.DOMParser : createHTMLParser();\n\nfunction RootNode (input, options) {\n  var root;\n  if (typeof input === 'string') {\n    var doc = htmlParser().parseFromString(\n      // DOM parsers arrange elements in the <head> and <body>.\n      // Wrapping in a custom element ensures elements are reliably arranged in\n      // a single element.\n      '<x-turndown id=\"turndown-root\">' + input + '</x-turndown>',\n      'text/html'\n    );\n    root = doc.getElementById('turndown-root');\n  } else {\n    root = input.cloneNode(true);\n  }\n  collapseWhitespace({\n    element: root,\n    isBlock: isBlock,\n    isVoid: isVoid,\n    isPre: options.preformattedCode ? isPreOrCode : null\n  });\n\n  return root\n}\n\nvar _htmlParser;\nfunction htmlParser () {\n  _htmlParser = _htmlParser || new HTMLParser();\n  return _htmlParser\n}\n\nfunction isPreOrCode (node) {\n  return node.nodeName === 'PRE' || node.nodeName === 'CODE'\n}\n\nfunction Node (node, options) {\n  node.isBlock = isBlock(node);\n  node.isCode = node.nodeName === 'CODE' || node.parentNode.isCode;\n  node.isBlank = isBlank(node);\n  node.flankingWhitespace = flankingWhitespace(node, options);\n  return node\n}\n\nfunction isBlank (node) {\n  return (\n    !isVoid(node) &&\n    !isMeaningfulWhenBlank(node) &&\n    /^\\s*$/i.test(node.textContent) &&\n    !hasVoid(node) &&\n    !hasMeaningfulWhenBlank(node)\n  )\n}\n\nfunction flankingWhitespace (node, options) {\n  if (node.isBlock || (options.preformattedCode && node.isCode)) {\n    return { leading: '', trailing: '' }\n  }\n\n  var edges = edgeWhitespace(node.textContent);\n\n  // abandon leading ASCII WS if left-flanked by ASCII WS\n  if (edges.leadingAscii && isFlankedByWhitespace('left', node, options)) {\n    edges.leading = edges.leadingNonAscii;\n  }\n\n  // abandon trailing ASCII WS if right-flanked by ASCII WS\n  if (edges.trailingAscii && isFlankedByWhitespace('right', node, options)) {\n    edges.trailing = edges.trailingNonAscii;\n  }\n\n  return { leading: edges.leading, trailing: edges.trailing }\n}\n\nfunction edgeWhitespace (string) {\n  var m = string.match(/^(([ \\t\\r\\n]*)(\\s*))(?:(?=\\S)[\\s\\S]*\\S)?((\\s*?)([ \\t\\r\\n]*))$/);\n  return {\n    leading: m[1], // whole string for whitespace-only strings\n    leadingAscii: m[2],\n    leadingNonAscii: m[3],\n    trailing: m[4], // empty for whitespace-only strings\n    trailingNonAscii: m[5],\n    trailingAscii: m[6]\n  }\n}\n\nfunction isFlankedByWhitespace (side, node, options) {\n  var sibling;\n  var regExp;\n  var isFlanked;\n\n  if (side === 'left') {\n    sibling = node.previousSibling;\n    regExp = / $/;\n  } else {\n    sibling = node.nextSibling;\n    regExp = /^ /;\n  }\n\n  if (sibling) {\n    if (sibling.nodeType === 3) {\n      isFlanked = regExp.test(sibling.nodeValue);\n    } else if (options.preformattedCode && sibling.nodeName === 'CODE') {\n      isFlanked = false;\n    } else if (sibling.nodeType === 1 && !isBlock(sibling)) {\n      isFlanked = regExp.test(sibling.textContent);\n    }\n  }\n  return isFlanked\n}\n\nvar reduce = Array.prototype.reduce;\nvar escapes = [\n  [/\\\\/g, '\\\\\\\\'],\n  [/\\*/g, '\\\\*'],\n  [/^-/g, '\\\\-'],\n  [/^\\+ /g, '\\\\+ '],\n  [/^(=+)/g, '\\\\$1'],\n  [/^(#{1,6}) /g, '\\\\$1 '],\n  [/`/g, '\\\\`'],\n  [/^~~~/g, '\\\\~~~'],\n  [/\\[/g, '\\\\['],\n  [/\\]/g, '\\\\]'],\n  [/^>/g, '\\\\>'],\n  [/_/g, '\\\\_'],\n  [/^(\\d+)\\. /g, '$1\\\\. ']\n];\n\nfunction TurndownService (options) {\n  if (!(this instanceof TurndownService)) return new TurndownService(options)\n\n  var defaults = {\n    rules: rules,\n    headingStyle: 'setext',\n    hr: '* * *',\n    bulletListMarker: '*',\n    codeBlockStyle: 'indented',\n    fence: '```',\n    emDelimiter: '_',\n    strongDelimiter: '**',\n    linkStyle: 'inlined',\n    linkReferenceStyle: 'full',\n    br: '  ',\n    preformattedCode: false,\n    blankReplacement: function (content, node) {\n      return node.isBlock ? '\\n\\n' : ''\n    },\n    keepReplacement: function (content, node) {\n      return node.isBlock ? '\\n\\n' + node.outerHTML + '\\n\\n' : node.outerHTML\n    },\n    defaultReplacement: function (content, node) {\n      return node.isBlock ? '\\n\\n' + content + '\\n\\n' : content\n    }\n  };\n  this.options = extend({}, defaults, options);\n  this.rules = new Rules(this.options);\n}\n\nTurndownService.prototype = {\n  /**\n   * The entry point for converting a string or DOM node to Markdown\n   * @public\n   * @param {String|HTMLElement} input The string or DOM node to convert\n   * @returns A Markdown representation of the input\n   * @type String\n   */\n\n  turndown: function (input) {\n    if (!canConvert(input)) {\n      throw new TypeError(\n        input + ' is not a string, or an element/document/fragment node.'\n      )\n    }\n\n    if (input === '') return ''\n\n    var output = process.call(this, new RootNode(input, this.options));\n    return postProcess.call(this, output)\n  },\n\n  /**\n   * Add one or more plugins\n   * @public\n   * @param {Function|Array} plugin The plugin or array of plugins to add\n   * @returns The Turndown instance for chaining\n   * @type Object\n   */\n\n  use: function (plugin) {\n    if (Array.isArray(plugin)) {\n      for (var i = 0; i < plugin.length; i++) this.use(plugin[i]);\n    } else if (typeof plugin === 'function') {\n      plugin(this);\n    } else {\n      throw new TypeError('plugin must be a Function or an Array of Functions')\n    }\n    return this\n  },\n\n  /**\n   * Adds a rule\n   * @public\n   * @param {String} key The unique key of the rule\n   * @param {Object} rule The rule\n   * @returns The Turndown instance for chaining\n   * @type Object\n   */\n\n  addRule: function (key, rule) {\n    this.rules.add(key, rule);\n    return this\n  },\n\n  /**\n   * Keep a node (as HTML) that matches the filter\n   * @public\n   * @param {String|Array|Function} filter The unique key of the rule\n   * @returns The Turndown instance for chaining\n   * @type Object\n   */\n\n  keep: function (filter) {\n    this.rules.keep(filter);\n    return this\n  },\n\n  /**\n   * Remove a node that matches the filter\n   * @public\n   * @param {String|Array|Function} filter The unique key of the rule\n   * @returns The Turndown instance for chaining\n   * @type Object\n   */\n\n  remove: function (filter) {\n    this.rules.remove(filter);\n    return this\n  },\n\n  /**\n   * Escapes Markdown syntax\n   * @public\n   * @param {String} string The string to escape\n   * @returns A string with Markdown syntax escaped\n   * @type String\n   */\n\n  escape: function (string) {\n    return escapes.reduce(function (accumulator, escape) {\n      return accumulator.replace(escape[0], escape[1])\n    }, string)\n  }\n};\n\n/**\n * Reduces a DOM node down to its Markdown string equivalent\n * @private\n * @param {HTMLElement} parentNode The node to convert\n * @returns A Markdown representation of the node\n * @type String\n */\n\nfunction process (parentNode) {\n  var self = this;\n  return reduce.call(parentNode.childNodes, function (output, node) {\n    node = new Node(node, self.options);\n\n    var replacement = '';\n    if (node.nodeType === 3) {\n      replacement = node.isCode ? node.nodeValue : self.escape(node.nodeValue);\n    } else if (node.nodeType === 1) {\n      replacement = replacementForNode.call(self, node);\n    }\n\n    return join(output, replacement)\n  }, '')\n}\n\n/**\n * Appends strings as each rule requires and trims the output\n * @private\n * @param {String} output The conversion output\n * @returns A trimmed version of the ouput\n * @type String\n */\n\nfunction postProcess (output) {\n  var self = this;\n  this.rules.forEach(function (rule) {\n    if (typeof rule.append === 'function') {\n      output = join(output, rule.append(self.options));\n    }\n  });\n\n  return output.replace(/^[\\t\\r\\n]+/, '').replace(/[\\t\\r\\n\\s]+$/, '')\n}\n\n/**\n * Converts an element node to its Markdown equivalent\n * @private\n * @param {HTMLElement} node The node to convert\n * @returns A Markdown representation of the node\n * @type String\n */\n\nfunction replacementForNode (node) {\n  var rule = this.rules.forNode(node);\n  var content = process.call(this, node);\n  var whitespace = node.flankingWhitespace;\n  if (whitespace.leading || whitespace.trailing) content = content.trim();\n  return (\n    whitespace.leading +\n    rule.replacement(content, node, this.options) +\n    whitespace.trailing\n  )\n}\n\n/**\n * Joins replacement to the current output with appropriate number of new lines\n * @private\n * @param {String} output The current conversion output\n * @param {String} replacement The string to append to the output\n * @returns Joined output\n * @type String\n */\n\nfunction join (output, replacement) {\n  var s1 = trimTrailingNewlines(output);\n  var s2 = trimLeadingNewlines(replacement);\n  var nls = Math.max(output.length - s1.length, replacement.length - s2.length);\n  var separator = '\\n\\n'.substring(0, nls);\n\n  return s1 + separator + s2\n}\n\n/**\n * Determines whether an input can be converted\n * @private\n * @param {String|HTMLElement} input Describe this parameter\n * @returns Describe what it returns\n * @type String|Object|Array|Boolean|Number\n */\n\nfunction canConvert (input) {\n  return (\n    input != null && (\n      typeof input === 'string' ||\n      (input.nodeType && (\n        input.nodeType === 1 || input.nodeType === 9 || input.nodeType === 11\n      ))\n    )\n  )\n}\n\n/* harmony default export */ const __WEBPACK_DEFAULT_EXPORT__ = (TurndownService);\n\n\n//# sourceURL=webpack://import-helpers/./node_modules/turndown/lib/turndown.browser.es.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./src/index.ts");
/******/ 	
/******/ })()
;