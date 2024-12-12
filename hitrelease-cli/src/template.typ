#set page(width: 10cm, height: 10cm)
#set page(fill: gradient.linear(color.oklch(90%, 0.3, {color_degree}deg, 100%), color.oklch(90%, 0.3, {color_degree_2}deg, 100%), angle: 45deg))

#align(center + horizon, text(size: 16pt, "{artist}"))
#align(center + horizon, text(size: 42pt, "{year}"))
#align(center + horizon, text(size: 16pt, style: "italic", "{title}"))

#place(
  top + right,
  float: true,
  scope: "parent",
  clearance: 0em,
)[
  #text(size: 8pt)[
    #highlight(fill: rgb("#00000020"), extent: 6pt, radius: 8pt)[{card_deck}]
  ]
]
#place(
  bottom + right,
  float: true,
  scope: "parent",
  clearance: 0em,
)[
  #text(size: 8pt, "{card_nr}")
]

#pagebreak()
#set page(fill: none)

#align(center + horizon, box(stroke: 5pt + gradient.linear(..color.map.rainbow), radius: 3.5cm, width: 7cm, height: 7cm, image("{qr_path}", width: 50%)))
