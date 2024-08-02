document.addEventListener("htmx:send-error", function(event) {
  console.log(event);
  const element = event.detail.elt; // the element (eg <button>) that dispatched the request
  const eventType = event.detail.requestConfig.triggeringEvent.type; // Eg: "click"
  const retry = function() { htmx.trigger(element, eventType) };
  setTimeout(retry, 1000);
});

