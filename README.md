# 1. Make your app
[Go to the Discord developers site](https://discordapp.com/developers/applications/me) and click New App to make your Rich Presence app:

![2018-07-09-14-37-29](img/2018-07-09-14-37-29.png)

The App Name will be appended after ``Playing``, for example:

![2018-07-09-22-57-46](img/2018-07-09-22-57-46.png)

will make your Discord status ``Playing with dogs``.

To add Rich Presence, scroll to the bottom and click enable Rich Presence:

![2018-07-09-14-39-38](img/2018-07-09-14-39-38.png)

# 2. Connecting using the Rich Presence Client
1. Delete the default App ID and replace it with your own.
2. Press Reconnect.
3. Wait until the status says ``Ready``.
4. You are connected!
# 3. Adding Text
You can set the state and details using the textboxes:

![2018-07-10-01-05-33](img/2018-07-10-01-05-33.png)
![2018-07-10-01-05-13](img/2018-07-10-01-05-13.png)

The time between updates option lets you specify how often updates happen, in seconds. You can also update manually using the update button.

# 4. Adding images
Scroll to the bottom of the app settings and you should find the assets section:

![2018-07-10-00-51-32](img/2018-07-10-00-51-32.png)

The Rich Presence Client only works with large, numbered images, like so:

![2018-07-10-00-53-11](img/2018-07-10-00-53-11.png)

Then you can configure the client for a slideshow using this:

![2018-07-10-01-01-49](img/2018-07-10-01-01-49.png)

You can disable the slideshow option to not auto increment the current image (which happens on updates).
Entering invalid values might crash the client, but will most likely do nothing.

# Cross platform and other features
I'm happy to add these if enough want them.

# Discord
https://discord.gg/8pK5sAY