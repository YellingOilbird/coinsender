"use strict";(self.webpackChunkfront_tc=self.webpackChunkfront_tc||[]).push([[841],{4841:function(e,t,n){n.r(t),n.d(t,{default:function(){return T}});var r=n(4165),a=n(5861),s=n(9439),c=n(2791),o=n(1332),l=n(1413),i=n(4925),u=n(6088),d=n(2965),f=n(2680),m=["className"],p=["overflow","overflowX","className"],h=["placement"],v=["isNumeric"],_=["isNumeric"],Z=function(){for(var e=arguments.length,t=new Array(e),n=0;n<e;n++)t[n]=arguments[n];return t.filter(Boolean).join(" ")},x=(0,f.k)({name:"TableStylesContext",errorMessage:"useTableStyles returned is 'undefined'. Seems you forgot to wrap the components in \"<Table />\" "}),b=(0,s.Z)(x,2),w=b[0],j=b[1],k=(0,u.Gp)((function(e,t){var n=(0,u.jC)("Table",e),r=(0,d.Lr)(e),a=r.className,s=(0,i.Z)(r,m);return c.createElement(w,{value:n},c.createElement(u.m$.table,(0,l.Z)({role:"table",ref:t,__css:n.table,className:Z("chakra-table",a)},s)))}));k.displayName="Table";var N=(0,u.Gp)((function(e,t){var n,r=e.overflow,a=e.overflowX,s=e.className,o=(0,i.Z)(e,p);return c.createElement(u.m$.div,(0,l.Z)((0,l.Z)({ref:t,className:Z("chakra-table__container",s)},o),{},{__css:{display:"block",whiteSpace:"nowrap",WebkitOverflowScrolling:"touch",overflowX:null!==(n=null!==r&&void 0!==r?r:a)&&void 0!==n?n:"auto",overflowY:"hidden",maxWidth:"100%"}}))}));(0,u.Gp)((function(e,t){var n=e.placement,r=void 0===n?"bottom":n,a=(0,i.Z)(e,h),s=j();return c.createElement(u.m$.caption,(0,l.Z)((0,l.Z)({},a),{},{ref:t,__css:(0,l.Z)((0,l.Z)({},s.caption),{},{captionSide:r})}))})).displayName="TableCaption";var E=(0,u.Gp)((function(e,t){var n=j();return c.createElement(u.m$.thead,(0,l.Z)((0,l.Z)({},e),{},{ref:t,__css:n.thead}))})),y=(0,u.Gp)((function(e,t){var n=j();return c.createElement(u.m$.tbody,(0,l.Z)((0,l.Z)({},e),{},{ref:t,__css:n.tbody}))})),g=((0,u.Gp)((function(e,t){var n=j();return c.createElement(u.m$.tfoot,(0,l.Z)((0,l.Z)({},e),{},{ref:t,__css:n.tfoot}))})),(0,u.Gp)((function(e,t){var n=e.isNumeric,r=(0,i.Z)(e,v),a=j();return c.createElement(u.m$.th,(0,l.Z)((0,l.Z)({},r),{},{ref:t,__css:a.th,"data-is-numeric":n}))}))),G=(0,u.Gp)((function(e,t){var n=j();return c.createElement(u.m$.tr,(0,l.Z)((0,l.Z)({role:"row"},e),{},{ref:t,__css:n.tr}))})),$=(0,u.Gp)((function(e,t){var n=e.isNumeric,r=(0,i.Z)(e,_),a=j();return c.createElement(u.m$.td,(0,l.Z)((0,l.Z)({role:"gridcell"},r),{},{ref:t,__css:a.td,"data-is-numeric":n}))})),S=n(184),T=function(){var e=(0,c.useState)(!1),t=(0,s.Z)(e,2),n=t[0],l=t[1];return(0,c.useEffect)((function(){function e(){return(e=(0,a.Z)((0,r.Z)().mark((function e(){var t;return(0,r.Z)().wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.next=2,window.contract.get_user_vault({account_id:window.accountId});case 2:t=e.sent,console.log(t),l(t);case 5:case"end":return e.stop()}}),e)})))).apply(this,arguments)}!function(){e.apply(this,arguments)}()}),[]),(0,S.jsxs)("div",{style:{marginTop:"150px"},children:[(0,S.jsx)(o.X6,{textAlign:"center",children:"\ud83c\udfe7 Current user Vault \ud83c\udfe7"}),(0,S.jsx)(N,{mt:"40px",children:(0,S.jsxs)(k,{variant:"striped",colorScheme:"teal",children:[(0,S.jsx)(E,{children:(0,S.jsxs)(G,{children:[(0,S.jsx)(g,{children:"Number of NEAR sends"}),(0,S.jsx)(g,{children:"total NEAR sended"}),(0,S.jsx)(g,{children:"tokens used"}),(0,S.jsx)(g,{isNumeric:!0,children:"tokens balances"})]})}),(0,S.jsx)(y,{children:n&&(0,S.jsxs)(G,{children:[(0,S.jsx)($,{children:null===n||void 0===n?void 0:n.near_sends_num}),(0,S.jsx)($,{children:null===n||void 0===n?void 0:n.total_near_amount_sent}),(0,S.jsx)($,{children:n.tokens_used.map((function(e){return e}))}),(0,S.jsx)($,{children:n.token_deposits.map((function(e,t){return(0,S.jsxs)(S.Fragment,{children:[n.tokens_used[t],"(",n.token_deposits[t][1],")"]})}))})]})})]})})]})}}}]);
//# sourceMappingURL=841.d6b15ab8.chunk.js.map