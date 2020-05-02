// Please kill me now
// This does pagination
// 4/1/20 Renamed to kawaii.sh, too lazy to rename the plugin though
(function ($) {
  $.fn.nekoPaginate = function (options) {
    var defaults = {
      paginateElement: 'li',
      hashPage: 'page',
      elementsPerPage: 10,
      effect: 'default',
      slideOffset: 200,
      firstButton: true,
      firstButtonText: '<<',
      lastButton: true,
      lastButtonText: '>>',
      prevButton: true,
      prevButtonText: 'Prev',
      nextButton: true,
      nextButtonText: 'Next'
    }

    return this.each(function (instance) {
      var plugin = {}
      plugin.el = $(this)
      plugin.el.addClass('nekoPaginateList')

      plugin.settings = {
        pages: 0,
        objElements: Object,
        currentPage: 1
      }

      var getNbOfPages = function () {
        return Math.ceil(plugin.settings.objElements.length / plugin.settings.elementsPerPage)
      }

      var displayNav = function () {
        htmlNav = '<div class="nekoPaginateNav">'

        if (plugin.settings.firstButton) {
          htmlNav += '<a href="#' + plugin.settings.hashPage + ':1" title="First page" rel="1" class="first">' + plugin.settings.firstButtonText + '</a>'
        }

        if (plugin.settings.prevButton) {
          htmlNav += '<a href="" title="Previous" rel="" class="prev">' + plugin.settings.prevButtonText + '</a>'
        }

        // Commented out because of overflow
        // for(i = 1;i <= plugin.settings.pages;i++) {
        //     htmlNav += '<a href="#'+plugin.settings.hashPage+':'+i+'" title="Page '+i+'" rel="'+i+'" class="page">'+i+'</a>';
        // };

        if (plugin.settings.nextButton) {
          htmlNav += '<a href="" title="Next" rel="" class="next">' + plugin.settings.nextButtonText + '</a>'
        }

        if (plugin.settings.lastButton) {
          htmlNav += '<a href="#' + plugin.settings.hashPage + ':' + plugin.settings.pages + '" title="Last page" rel="' + plugin.settings.pages + '" class="last">' + plugin.settings.lastButtonText + '</a>'
        }

        htmlNav += '</div>'
        plugin.nav = $(htmlNav)
        plugin.el.after(plugin.nav)

        var elSelector = '#' + plugin.el.get(0).id + ' + '
        $(elSelector + ' .nekoPaginateNav a.page,' +
                elSelector + ' .nekoPaginateNav a.first,' +
                elSelector + ' .nekoPaginateNav a.last').on('click', function (e) {
          e.preventDefault()
          displayPage($(this).attr('rel'))
        })

        $(elSelector + ' .nekoPaginateNav a.prev').on('click', function (e) {
          e.preventDefault()
          page = plugin.settings.currentPage > 1 ? parseInt(plugin.settings.currentPage) - 1 : 1
          displayPage(page)
        })

        $(elSelector + ' .nekoPaginateNav a.next').on('click', function (e) {
          e.preventDefault()
          page = plugin.settings.currentPage < plugin.settings.pages ? parseInt(plugin.settings.currentPage) + 1 : plugin.settings.pages
          displayPage(page)
        })
      }

      var displayPage = function (page, forceEffect) {
        if (plugin.settings.currentPage != page) {
          plugin.settings.currentPage = parseInt(page)
          offsetStart = (page - 1) * plugin.settings.elementsPerPage
          offsetEnd = page * plugin.settings.elementsPerPage
          if (typeof (forceEffect) !== 'undefined') {
            eval('transition_' + forceEffect + '(' + offsetStart + ', ' + offsetEnd + ')')
          } else {
            eval('transition_' + plugin.settings.effect + '(' + offsetStart + ', ' + offsetEnd + ')')
          }

          plugin.nav.find('.current').removeClass('current')
          plugin.nav.find('a.page:eq(' + (page - 1) + ')').addClass('current')

          switch (plugin.settings.currentPage) {
            case 1:
              $('.nekoPaginateNav a', plugin).removeClass('disabled')
              $('.nekoPaginateNav a.first, .nekoPaginateNav a.prev', plugin).addClass('disabled')
              break
            case plugin.settings.pages:
              $('.nekoPaginateNav a', plugin).removeClass('disabled')
              $('.nekoPaginateNav a.last, .nekoPaginateNav a.next', plugin).addClass('disabled')
              break
            default:
              $('.nekoPaginateNav a', plugin).removeClass('disabled')
              break
          }
        }
      }

      var transition_default = function (offsetStart, offsetEnd) {
        plugin.currentElements.hide()
        plugin.currentElements = plugin.settings.objElements.slice(offsetStart, offsetEnd).clone()
        plugin.el.html(plugin.currentElements)
        plugin.currentElements.show()
      }

      plugin.settings = $.extend({}, defaults, options)

      plugin.currentElements = $([])
      plugin.settings.objElements = plugin.el.find(plugin.settings.paginateElement)
      plugin.settings.pages = getNbOfPages()
      if (plugin.settings.pages > 1) {
        plugin.el.html()

        // Here we go
        displayNav()

        page = 1
        if (document.location.hash.indexOf('#' + plugin.settings.hashPage + ':') != -1) {
          page = parseInt(document.location.hash.replace('#' + plugin.settings.hashPage + ':', ''))
          if (page.length <= 0 || page < 1 || page > plugin.settings.pages) {
            page = 1
          }
        }

        displayPage(page, 'default')
        window.pageobj = plugin.settings.objElements
      }
    })
  }
})(jQuery)
