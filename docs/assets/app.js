    (function () {
      "use strict";

      var body = document.body;
      var toggle = document.querySelector(".menu-toggle");
      var themeToggle = document.querySelector(".theme-toggle");
      var backdrop = document.querySelector(".backdrop");
      var navLinks = Array.prototype.slice.call(document.querySelectorAll(".nav-link"));
      var sections = navLinks
        .map(function (link) {
          var href = link.getAttribute("href") || "";
          return href.charAt(0) === "#" ? document.querySelector(href) : null;
        })
        .filter(Boolean);

      var themeStorageKey = "tg_transfer_bot_docs_theme";

      function getPreferredTheme() {
        return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
      }

      function updateThemeToggle(theme) {
        var isDark = theme === "dark";
        themeToggle.querySelector("span").textContent = isDark ? "☀" : "☾";
        themeToggle.setAttribute("aria-pressed", String(isDark));
        themeToggle.setAttribute("aria-label", isDark ? "切换到浅色模式" : "切换到深色模式");
        themeToggle.setAttribute("title", isDark ? "切换到浅色模式" : "切换到深色模式");
      }

      function setTheme(theme, persist) {
        document.documentElement.dataset.theme = theme;
        updateThemeToggle(theme);
        if (persist) {
          try {
            window.localStorage.setItem(themeStorageKey, theme);
          } catch (error) {
            // 浏览器隐私模式可能禁用本地存储，主题仍在当前页面生效。
          }
        }
      }

      var savedTheme;
      try {
        savedTheme = window.localStorage.getItem(themeStorageKey);
      } catch (error) {
        savedTheme = null;
      }
      if (savedTheme === "dark" || savedTheme === "light") {
        setTheme(savedTheme, false);
      } else {
        updateThemeToggle(getPreferredTheme());
      }

      window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", function (event) {
        if (!document.documentElement.dataset.theme) {
          updateThemeToggle(event.matches ? "dark" : "light");
        }
      });

      themeToggle.addEventListener("click", function () {
        var activeTheme = document.documentElement.dataset.theme || getPreferredTheme();
        setTheme(activeTheme === "dark" ? "light" : "dark", true);
      });

      function setNavigation(open) {
        body.classList.toggle("nav-open", open);
        toggle.setAttribute("aria-expanded", String(open));
        toggle.setAttribute("aria-label", open ? "关闭文档目录" : "打开文档目录");
      }

      toggle.addEventListener("click", function () {
        setNavigation(!body.classList.contains("nav-open"));
      });

      backdrop.addEventListener("click", function () {
        setNavigation(false);
      });

      navLinks.forEach(function (link) {
        link.addEventListener("click", function () {
          setNavigation(false);
        });
      });

      document.addEventListener("keydown", function (event) {
        if (event.key === "Escape") {
          setNavigation(false);
        }
      });

      var observer = new IntersectionObserver(
        function (entries) {
          var visible = entries
            .filter(function (entry) { return entry.isIntersecting; })
            .sort(function (left, right) { return right.intersectionRatio - left.intersectionRatio; });
          if (!visible.length) {
            return;
          }
          var id = "#" + visible[0].target.id;
          navLinks.forEach(function (link) {
            link.classList.toggle("active", link.getAttribute("href") === id);
          });
        },
        { rootMargin: "-18% 0px -68% 0px", threshold: [0, 0.15, 0.5] }
      );

      sections.forEach(function (section) {
        observer.observe(section);
      });

      document.querySelectorAll(".copy-button").forEach(function (button) {
        button.addEventListener("click", function () {
          var code = button.parentElement.querySelector("code");
          if (!code) {
            return;
          }
          navigator.clipboard.writeText(code.textContent).then(function () {
            button.textContent = "已复制";
            button.classList.add("copied");
            window.setTimeout(function () {
              button.textContent = "复制";
              button.classList.remove("copied");
            }, 1600);
          }).catch(function () {
            button.textContent = "复制失败";
            window.setTimeout(function () {
              button.textContent = "复制";
            }, 1600);
          });
        });
      });

    })();
