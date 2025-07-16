package ru.yumiyulili.hexnicks

import org.bukkit.ChatColor
import java.util.regex.Pattern

object Color {
    private val pattern: Pattern = Pattern.compile("#[a-fA-F0-9]{6}")

    fun color(from: String): String {
        var from = from
        var matcher = pattern.matcher(from)
        while (matcher.find()) {
            val hexCode = from.substring(matcher.start(), matcher.end())
            val replaceSharp = hexCode.replace('#', 'x')
            val ch = replaceSharp.toCharArray()
            val builder = StringBuilder()
            for (c in ch) {
                builder.append("&").append(c)
            }
            from = from.replace(hexCode, builder.toString())
            matcher = pattern.matcher(from)
        }

        return ChatColor.translateAlternateColorCodes('&', from)
    }
}
