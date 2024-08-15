import { Telegraf, Markup } from "telegraf"
import { config } from "dotenv"

config()

const URL = process.env.TELEGRAM_BOT_API

const bot = new Telegraf(URL)

bot.use(Telegraf.log())

bot.command("start", async ctx => {
    await ctx.replyWithHTML(`<i>Hello ${ctx.message.from.username} 👋, </i>\n\n<b>Welcome to the best Web3 Auction dApp where you can sell your prized possesions for massive profits 💰.</b>\n\n<i>Powered by Raptor 👾\nbuilt on Solana 🤖.</i>`)
})

bot.command("auction", async ctx => {
    try { 
        if (ctx.message.chat.type != "private") {
            

            await ctx.replyWithHTML(
                `<i>Hello ${ctx.message.from.username} 👋, </i>\n\n<b>Wanna start an auction, Follow the instructions below</b>`,
                {
                    parse_mode : "HTML",
                    ...Markup.inlineKeyboard([
                        [Markup.button.callback("Enter the auction name ✅", "name")]
                    ])
                }
            )
        } else {
            await ctx.reply("⚠️ Add this bot to a group to begin using it.")
        }
    } catch (err) {
        await ctx.replyWithHTML(`<b>🚫 Sorry for the Inconveniences.</b>`)
    }
})

connectDB()

bot.launch()