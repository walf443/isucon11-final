(window.webpackJsonp=window.webpackJsonp||[]).push([[18,6],{244:function(t,e,n){"use strict";n.r(e);var r=n(0).a.extend({props:{color:{type:String,default:"plain"},type:{type:String,default:"button"},disabled:{type:Boolean,default:!1}},computed:{colorType:function(){return"primary"===this.color?["bg-primary-500","text-white"]:["bg-white","border-primary-500","text-primary-500"]}}}),l=n(20),component=Object(l.a)(r,(function(){var t=this,e=t.$createElement;return(t._self._c||e)("button",{staticClass:"\n    py-2\n    px-6\n    border\n    rounded\n    disabled:bg-gray-400 disabled:text-white disabled:border-gray-400\n  ",class:t.colorType,attrs:{type:t.type,disabled:t.disabled},on:{click:function(e){return t.$emit("click")}}},[t._t("default")],2)}),[],!1,null,null,null);e.default=component.exports;installComponents(component,{Button:n(244).default})},276:function(t,e,n){"use strict";n.r(e);var r=n(0).a.extend({props:{tabs:{type:Array,required:!0}},data:function(){return{activeTabIndex:0}},computed:{activeSlotName:function(){return this.tabs[this.activeTabIndex].id}},methods:{tabClasses:function(t){return t===this.activeTabIndex?["text-primary-500","font-bold","border-primary-500"]:["text-gray-600","border-gray-600"]},activate:function(t){this.activeTabIndex=t}}}),l=n(20),component=Object(l.a)(r,(function(){var t=this,e=t.$createElement,n=t._self._c||e;return n("div",{staticClass:"bg-white"},[n("nav",{staticClass:"flex flex-col sm:flex-row"},t._l(t.tabs,(function(e,r){return n("button",{key:e.id,staticClass:"\n        py-4\n        px-6\n        block\n        hover:text-primary-500\n        focus:outline-none\n        border-b-4\n      ",class:t.tabClasses(r),on:{click:function(e){return t.activate(r)}}},[t._v("\n      "+t._s(e.label)+"\n    ")])})),0),t._v(" "),t._t(t.activeSlotName)],2)}),[],!1,null,null,null);e.default=component.exports;installComponents(component,{Button:n(244).default})}}]);