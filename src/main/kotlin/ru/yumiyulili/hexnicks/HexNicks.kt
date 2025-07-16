package ru.yumiyulili.hexnicks

import org.bukkit.Bukkit
import org.bukkit.entity.Player
import org.bukkit.event.EventHandler
import org.bukkit.event.Listener
import org.bukkit.event.player.PlayerJoinEvent
import org.bukkit.plugin.java.JavaPlugin
import ru.destroy.RustWrapper


class HexNicks : JavaPlugin() , Listener {

    override fun onEnable() {
        Bukkit.getPluginManager().registerEvents(this,this)

    }

    @EventHandler
    fun name(e: PlayerJoinEvent) {
        var user = e.player.name
        var color = Color.color(RustWrapper.hexColorOf(user));
        var player = e.player
        player.setDisplayName(color.plus(user))
        player.setPlayerListName(color.plus(user))
    }


    override fun onDisable() {
        // Plugin shutdown logic
    }
}