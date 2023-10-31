# Dattebayo! A forum-based, play-by-post Naruto RPG

Dattebayo! is a Play-by-Post RPG that extends the universe of Naruto, created by Masashi Kishimoto.

Play-by-post RPGs are online versions of tabletop role-playing games, where players take on the roles of characters and participate in a story that is written collaboratively through a series of posts in a forum or on a message board. Each player takes turns making posts that describe the actions and decisions of their character, and the story progresses as players respond to each other's posts. This format allows for a more leisurely pace and a more detailed, text-based form of role-playing than is possible in real-time games.

# Tech Stack

| Purpose           | Technology   |
|------------------|--------------|
| 🛠 Backend | Rust         |
| 🌐 HTTP Server      | Axum         |
| 📊 Database Typecheck| sqlx         |
| 🗄 Database         | PostgreSQL   |
| 🔐 Authentication   | Hanko        |
| 🖼 Frontend Framework| Next.js      |
| ✍️ Frontend Language | TypeScript   |
| 🚀 Frontend Deploy  | Vercel       |
| 🐳 Backend Deploy   | Docker + VPS¹ |

¹: For the current PoC, it will be hosted on Railway; Later on, as Railway does not have SA region, I will move to a SA VPS.

# Upcoming changes

31 Oct: I had some issues during the last week October, and I couldn't finish the project on time for the Hackathon. However, the project will continue either way. Here are some changes to make it usable:

- [ ] Rewrite my Axum middleware's logic to include user information. There are some good examples on GitHub, but the way I've done it is not the best way, I will rewrite it entirely.
- [ ] Improve UI responsiveness.
- [ ] Implement all the core features of the forum that are missing:
- - [ ] User profile
- - [ ] Create new topics and new posts
- - [ ] Edit posts, topics and sections.
- - [ ] Role management and permissions.

With those changes, the forum will be usable, serving as a great way to estimate future database usage. I will also be able to start coding the game rules and the game itself.

# Contributing

For the moment, I'm not accepting contributions. I'm participating on Hanko Hackathon. I was searching for a good option for my app authentication and I stumbled into their product. After deciding to go for it, I've noticed that they were hosting a hackathon, so I'm going for it for this month of October. After the Hackathon, I will move my project to an organisation here on GitHub, so other friends and the public can contribute. I'm not opening now because I don't think it would be fair.

# Dattebayo! Backstory

Dattebayo! started roughly two years ago, when me and my friends decided to open our own forum. We spent most of the time writing the actual game rules. Life happens and we had two hiatus, as we are adults that need to work. This time, I'm dead serious about it, as this is something we love. (and we are desperate to start playing it)

# License notice

This is being licensed and always will be as AGPL 3.0 (or any other that may come to improve and contribute to the free software movement). All the code aspect is under the 4 freedoms we have. There will be one submodule that will remain private, as its core is not related to code. You are free to use it as you want, **as long as you keep your code open for the world to use it**!