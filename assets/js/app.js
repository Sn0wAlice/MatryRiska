try {
    var dropdownMenus = document.querySelectorAll(".dropdown-menu.stop");
    dropdownMenus.forEach(function (e) {
        e.addEventListener("click", function (e) {
            e.stopPropagation()
        })
    })
} catch (e) {}
try {
    lucide.createIcons()
} catch (e) {}
try {
    var themeColorToggle = document.getElementById("light-dark-mode");
    themeColorToggle && themeColorToggle.addEventListener("click", function (e) {
        "light" === document.documentElement.getAttribute("data-bs-theme") ? document.documentElement.setAttribute("data-bs-theme", "dark") : document.documentElement.setAttribute("data-bs-theme", "light")
    })
} catch (e) {}
try {
    var collapsedToggle = document.querySelector(".mobile-menu-btn");
    const h = document.querySelector(".startbar-overlay"),
        changeSidebarSize = (collapsedToggle ? .addEventListener("click", function () {
            "collapsed" == document.body.getAttribute("data-sidebar-size") ? document.body.setAttribute("data-sidebar-size", "default") : document.body.setAttribute("data-sidebar-size", "collapsed")
        }), h && h.addEventListener("click", () => {
            document.body.setAttribute("data-sidebar-size", "collapsed")
        }), () => {
            310 <= window.innerWidth && window.innerWidth <= 1440 ? document.body.setAttribute("data-sidebar-size", "collapsed") : document.body.setAttribute("data-sidebar-size", "default")
        });
    window.addEventListener("resize", () => {
        changeSidebarSize()
    }), changeSidebarSize()
} catch (e) {}
try {
    const k = document.querySelectorAll('[data-bs-toggle="tooltip"]'),
        l = [...k].map(e => new bootstrap.Tooltip(e));
    var popoverTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="popover"]')),
        popoverList = popoverTriggerList.map(function (e) {
            return new bootstrap.Popover(e)
        })
} catch (e) {}
try {
    changeSidebarSize(), window.addEventListener("resize", changeSidebarSize), window.addEventListener("resize", () => {
        changeSidebarSize()
    }), changeSidebarSize()
} catch (e) {}

function windowScroll() {
    var e = document.getElementById("topbar-custom");
    null != e && (50 <= document.body.scrollTop || 50 <= document.documentElement.scrollTop ? e.classList.add("nav-sticky") : e.classList.remove("nav-sticky"))
}
window.addEventListener("scroll", e => {
    e.preventDefault(), windowScroll()
});
const initVerticalMenu = () => {
    var e = document.querySelectorAll(".navbar-nav li .collapse");
    document.querySelectorAll(".navbar-nav li [data-bs-toggle='collapse']").forEach(e => {
        e.addEventListener("click", function (e) {
            e.preventDefault()
        })
    }), e.forEach(e => {
        e.addEventListener("show.bs.collapse", function (t) {
            const o = t.target.closest(".collapse.show");
            document.querySelectorAll(".navbar-nav .collapse.show").forEach(e => {
                e !== t.target && e !== o && new bootstrap.Collapse(e).hide()
            })
        })
    }), document.querySelector(".navbar-nav") && (document.querySelectorAll(".navbar-nav a").forEach(function (t) {
        var e = window.location.href.split(/[?#]/)[0];
        if (t.href === e) {
            t.classList.add("active"), t.parentNode.classList.add("active");
            let e = t.closest(".collapse");
            for (; e;) e.classList.add("show"), e.parentElement.children[0].classList.add("active"), e.parentElement.children[0].setAttribute("aria-expanded", "true"), e = e.parentElement.closest(".collapse")
        }
    }), setTimeout(function () {
        var e, a, n, r, c, l, t = document.querySelector(".nav-item li a.active");

        function d() {
            e = l += 20, t = r, o = c;
            var e, t, o = (e /= n / 2) < 1 ? o / 2 * e * e + t : -o / 2 * (--e * (e - 2) - 1) + t;
            a.scrollTop = o, l < n && setTimeout(d, 20)
        }
        null != t && (e = document.querySelector(".main-nav .simplebar-content-wrapper"), t = t.offsetTop - 300, e) && 100 < t && (n = 600, r = (a = e).scrollTop, c = t - r, l = 0, d())
    }, 200))
};
initVerticalMenu();