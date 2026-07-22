    (function () {
      "use strict";

      var body = document.body;
      var toggle = document.querySelector(".menu-toggle");
      var backdrop = document.querySelector(".backdrop");
      var navLinks = Array.prototype.slice.call(document.querySelectorAll(".nav-link"));
      var sections = navLinks
        .map(function (link) {
          var href = link.getAttribute("href") || "";
          return href.charAt(0) === "#" ? document.querySelector(href) : null;
        })
        .filter(Boolean);

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
