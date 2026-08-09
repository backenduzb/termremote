import asyncio
import json
import sys

SERVER_HOST = "zephyr.proxy.rlwy.net"
SERVER_PORT = 14533

async def receive_messages(reader):
    while True:
        try:
            line = await reader.readline()
            if not line:
                print("\nServer bilan aloqa uzildi.")
                break

            packet = json.loads(line.decode('utf-8').strip())
            sender = packet.get("from", "Noma'lum")
            payload = packet.get("payload", "")
            
            print(f"\n\r[Xabar - {sender}]: {payload}\n> ", end="")
            sys.stdout.flush()
        except Exception as e:
            print(f"\nXabar o'qishda xatolik: {e}")
            break

async def send_messages(writer, user_id):
    init_packet = {
        "sender_id": user_id,
        "target_id": "system",
        "payload": "online"
    }
    writer.write((json.dumps(init_packet) + "\n").encode('utf-8'))
    await writer.drain()

    while True:
        msg = await asyncio.to_thread(input, "> ")
        msg = msg.strip()

        if msg.lower() == 'exit':
            break

        if ":" not in msg:
            print("⚠️ Xato format! Format ushbu ko'rinishda bo'lsin -> target_id:xabar")
            continue

        target_id, payload = msg.split(":", 1)
        packet = {
            "sender_id": user_id,
            "target_id": target_id.strip(),
            "payload": payload.strip()
        }

        # TCP streamida xabar ajratuvchi sifatida \n yuboriladi
        writer.write((json.dumps(packet) + "\n").encode('utf-8'))
        await writer.drain()

    writer.close()
    await writer.wait_closed()

async def main():
    user_id = input("O'z ID-ingizni kiriting (masalan, user_a): ").strip()
    if not user_id:
        print("ID kiritilmadi!")
        return

    try:
        reader, writer = await asyncio.open_connection(SERVER_HOST, SERVER_PORT)
        print(f"=== Serverga ({SERVER_HOST}:{SERVER_PORT}) {user_id} sifatida ulandingiz ===")
        print("Xabar yuborish formati: TARGET_ID:XABAR (masalan, user_b:Salom)")
        print("Chiqish uchun 'exit' deb yozing.\n")

        await asyncio.gather(
            receive_messages(reader),
            send_messages(writer, user_id)
        )
    except Exception as e:
        print(f"Serverga ulanishda xatolik: {e}")

if __name__ == "__main__":
    asyncio.run(main())