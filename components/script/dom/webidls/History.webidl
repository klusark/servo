interface History {
  readonly attribute long length;
  // readonly attribute any state;
  void go(optional long delta);
  void back();
  void forward();
  void pushState(any data, DOMString title, optional DOMString? url = null);
  void replaceState(any data, DOMString title, optional DOMString? url = null);
};
