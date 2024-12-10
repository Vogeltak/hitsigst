#set page(width: 10cm, height: 10cm)
#set page(fill: gradient.linear(color.hsl({color_degree}deg, 100%, 85%), color.hsl(-{color_degree}deg, 100%, 65%), angle: 45deg))

#align(center + horizon, text(size: 16pt, "{artist}"))
#align(center + horizon, text(size: 42pt, "{year}"))
#align(center + horizon, text(size: 16pt, style: "italic", "{title}"))

#place(
  top + right,
  float: true,
  scope: "parent",
  clearance: 0em,
)[
  #text(size: 8pt, "{card_deck}")
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

#align(center + horizon, image("{qr_path}", width: 80%))
