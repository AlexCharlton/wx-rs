#include <wx/wx.h>
#include "wx/clipbrd.h"
#include <iostream>

#if defined(_WIN32) || defined(_WIN64) || defined(WIN64) || defined(WIN32) || defined(__MINGW32__) || defined(__MINGW64__)
#define __WINDOWS__
#endif

#ifdef __WINDOWS__
  #include <windows.h>
#endif

#ifdef __WINDOWS__
void setDPIAware() {
  // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware
  HMODULE module = LoadLibraryA("User32.dll");
  if(module) {
    // BOOL(WINAPI * pFunc)(void) =(BOOL(WINAPI *)(void)) GetProcAddress(module, "SetProcessDPIAware");

    // if(pFunc) {
    //   pFunc();
    // }

    BOOL(WINAPI * pFunc2)(DPI_AWARENESS_CONTEXT) =(BOOL(WINAPI *)(DPI_AWARENESS_CONTEXT)) GetProcAddress(module, "SetProcessDpiAwarenessContext");

    if(pFunc2) {
      pFunc2(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }

    FreeLibrary(module);
  }
}
#else
  void setDPIAware() {}
#endif

enum cursor {
  NO_CURSOR,
  ARROW_CURSOR,
  CROSS_CURSOR,
  HAND_CURSOR,
  IBEAM_CURSOR,
  MAGNIFY_CURSOR,
  NO_ENTRY_CURSOR,
  PENCIL_CURSOR,
  SIZE_CURSOR,
  SIZE_NESW_CURSOR,
  SIZE_NS_CURSOR,
  SIZE_NWSE_CURSOR,
  SIZE_WE_CURSOR,
  NUMBER_OF_CURSORS
};

wxStockCursor cursorToStock(cursor c) {
  if (c == ARROW_CURSOR) return wxCURSOR_ARROW;
  else if (c == NO_CURSOR) return wxCURSOR_BLANK;
  else if (c == CROSS_CURSOR) return wxCURSOR_CROSS;
  else if (c == HAND_CURSOR) return wxCURSOR_HAND;
  else if (c == IBEAM_CURSOR) return wxCURSOR_IBEAM;
  else if (c == MAGNIFY_CURSOR) return wxCURSOR_MAGNIFIER;
  else if (c == PENCIL_CURSOR) return wxCURSOR_PENCIL;
  else if (c == SIZE_CURSOR) return wxCURSOR_SIZING;
  else if (c == SIZE_NESW_CURSOR) return wxCURSOR_SIZENESW;
  else if (c == SIZE_NS_CURSOR) return wxCURSOR_SIZENS;
  else if (c == SIZE_NWSE_CURSOR) return wxCURSOR_SIZENWSE;
  else if (c == SIZE_WE_CURSOR) return wxCURSOR_SIZEWE;
  else if (c == NO_ENTRY_CURSOR) return wxCURSOR_NO_ENTRY;
  else return wxCURSOR_ARROW;
}

class MyFrame: public wxFrame {
public:
  MyFrame(const wxString& title, const wxSize& size);
  ~MyFrame();

  wxCursor *cursors[NUMBER_OF_CURSORS];
  wxTimer *timer;

  void (*render)();
  void (*handle_events)(wxEvent &event);

  void OnPaint(wxPaintEvent &event);
  void OnEvent(wxEvent &event);
  void OnTimer(wxTimerEvent& event);
  void OnClose(wxCloseEvent &event);

  void BindEvents();
  void InitCursors();

  wxWindow *inputWin;
  wxDECLARE_EVENT_TABLE();
};


enum {
  TIMER_ID = 1,
};

wxBEGIN_EVENT_TABLE(MyFrame, wxFrame)
EVT_PAINT(MyFrame::OnPaint)
EVT_CLOSE(MyFrame::OnClose)
EVT_TIMER(TIMER_ID, MyFrame::OnTimer)
wxEND_EVENT_TABLE()

void MyFrame::OnTimer(wxTimerEvent& event)
{
  if (this->render) {
    this->render();
  }
  if (this->handle_events) {
    this->handle_events(event);
  }
}

void MyFrame::OnPaint( wxPaintEvent& WXUNUSED(event) ){
  wxPaintDC dc(this);

  if (this->render) {
    this->render();
  }
}


MyFrame::MyFrame(const wxString& title, const wxSize& size)
  : wxFrame(NULL, wxID_ANY, title, wxDefaultPosition, size)
{
  // Needed to get App to catch key events on OSX
  inputWin = new wxWindow(this, wxID_ANY,
                          wxDefaultPosition,
                          wxSize(-1, -1),
                          wxBORDER_NONE | wxTRANSPARENT_WINDOW);

  InitCursors();
  Show(true);

  this->timer = new wxTimer(this, TIMER_ID);
  timer->Start(5);
}

MyFrame::~MyFrame() {
  // Needed for graceful exit in Windows
  wxTheApp->Unbind(wxEVT_MOTION, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_LEFT_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_LEFT_UP, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_LEFT_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_MIDDLE_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_MIDDLE_UP, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_MIDDLE_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_RIGHT_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_RIGHT_UP, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_RIGHT_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_AUX1_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_AUX1_UP, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_AUX1_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_AUX2_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_AUX2_UP, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_AUX2_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_MOUSEWHEEL, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_KEY_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_KEY_UP, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_SIZE, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_MOVE, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_ACTIVATE, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_LEAVE_WINDOW, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_ENTER_WINDOW, &MyFrame::OnEvent, this);
  wxTheApp->Unbind(wxEVT_MENU, &MyFrame::OnEvent, this);
}

void MyFrame::OnEvent(wxEvent &event){
  if (this->handle_events) {
    this->handle_events(event);
  }
}

void MyFrame::BindEvents() {
  wxTheApp->Bind(wxEVT_MOTION, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_LEFT_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_LEFT_UP, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_LEFT_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_MIDDLE_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_MIDDLE_UP, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_MIDDLE_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_RIGHT_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_RIGHT_UP, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_RIGHT_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_AUX1_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_AUX1_UP, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_AUX1_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_AUX2_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_AUX2_UP, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_AUX2_DCLICK, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_MOUSEWHEEL, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_KEY_DOWN, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_KEY_UP, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_SIZE, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_MOVE, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_ACTIVATE, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_LEAVE_WINDOW, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_ENTER_WINDOW, &MyFrame::OnEvent, this);
  wxTheApp->Bind(wxEVT_MENU, &MyFrame::OnEvent, this);
}

void MyFrame::OnClose(wxCloseEvent& event)
{
  if (this->handle_events) {
    this->handle_events(event);
  }
  timer->Stop();
  event.Skip();
}

void MyFrame::InitCursors() {
  for (int i = 0; i < NUMBER_OF_CURSORS; i++) {
    cursors[i] = new wxCursor(cursorToStock((cursor) i));
  }
}

enum bridge_event_type {
  UNKNOWN_EVENT,
  MOUSE_MOTION,
  MOUSE_LEFT_DOWN,
  MOUSE_LEFT_UP,
  MOUSE_MIDDLE_DOWN,
  MOUSE_MIDDLE_UP,
  MOUSE_RIGHT_DOWN,
  MOUSE_RIGHT_UP,
  MOUSE_AUX1_DOWN,
  MOUSE_AUX1_UP,
  MOUSE_AUX2_DOWN,
  MOUSE_AUX2_UP,
  MOUSE_RIGHT_DCLICK,
  MOUSE_LEFT_DCLICK,
  MOUSE_MIDDLE_DCLICK,
  MOUSE_AUX1_DCLICK,
  MOUSE_AUX2_DCLICK,
  MOUSE_WHEEL,
  MOUSE_ENTER_WINDOW,
  MOUSE_LEAVE_WINDOW,
  KEY_DOWN,
  KEY_UP,
  RESIZE,
  WINDOW_MOVE,
  FOCUS,
  TIMER,
  EXIT,
  MENU,
};

int wxEventTypeToBridgeEventType(wxEventType t) {
  if (t == wxEVT_MOTION) return MOUSE_MOTION;
  else if (t == wxEVT_LEFT_DOWN) return MOUSE_LEFT_DOWN;
  else if (t == wxEVT_LEFT_UP) return MOUSE_LEFT_UP;
  else if (t == wxEVT_LEFT_DCLICK) return MOUSE_LEFT_DCLICK;
  else if (t == wxEVT_MIDDLE_DOWN) return MOUSE_MIDDLE_DOWN;
  else if (t == wxEVT_MIDDLE_UP) return MOUSE_MIDDLE_UP;
  else if (t == wxEVT_MIDDLE_DCLICK) return MOUSE_MIDDLE_DCLICK;
  else if (t == wxEVT_RIGHT_DOWN) return MOUSE_RIGHT_DOWN;
  else if (t == wxEVT_RIGHT_UP) return MOUSE_RIGHT_UP;
  else if (t == wxEVT_RIGHT_DCLICK) return MOUSE_RIGHT_DCLICK;
  else if (t == wxEVT_AUX1_DOWN) return MOUSE_AUX1_DOWN;
  else if (t == wxEVT_AUX1_UP) return MOUSE_AUX1_UP;
  else if (t == wxEVT_AUX1_DCLICK) return MOUSE_AUX1_DCLICK;
  else if (t == wxEVT_AUX2_DOWN) return MOUSE_AUX2_DOWN;
  else if (t == wxEVT_AUX2_UP) return MOUSE_AUX2_UP;
  else if (t == wxEVT_AUX2_DCLICK) return MOUSE_AUX2_DCLICK;
  else if (t == wxEVT_MOUSEWHEEL) return MOUSE_WHEEL;
  else if (t == wxEVT_ENTER_WINDOW) return MOUSE_ENTER_WINDOW;
  else if (t == wxEVT_LEAVE_WINDOW) return MOUSE_LEAVE_WINDOW;
  else if (t == wxEVT_KEY_DOWN) return KEY_DOWN;
  else if (t == wxEVT_KEY_UP) return KEY_UP;
  else if (t == wxEVT_SIZE) return RESIZE;
  else if (t == wxEVT_MOVE) return WINDOW_MOVE;
  else if (t == wxEVT_ACTIVATE) return FOCUS;
  else if (t == wxEVT_TIMER) return TIMER;
  else if (t == wxEVT_CLOSE_WINDOW) return EXIT;
  else if (t == wxEVT_MENU) return MENU;
  else return UNKNOWN_EVENT;
}

class MyApp: public wxApp {
public:
  MyApp(const wxString& title, const wxSize& size);
  MyFrame *frame;
  virtual bool OnInit();
private:
  wxString title;
  wxSize size;
};

bool MyApp::OnInit() {
  frame = new MyFrame( title, size );
  frame->Show( true );
  return true;
}

MyApp::MyApp(const wxString& t, const wxSize& s) : title(t), size(s) {}

MyApp& wxGetApp() { return *static_cast<MyApp*>(wxApp::GetInstance()); }

class CustomDataObject : public wxDataObjectSimple
{
public:
  CustomDataObject(): wxDataObjectSimple(wxDataFormat("wx_bridge/custom")) {}

  virtual size_t GetDataSize() const;
  virtual bool GetDataHere(void *buf) const;
  virtual bool SetData(size_t len, const void* buf);

private:
  size_t size;
  const void *buffer;
};

size_t CustomDataObject::GetDataSize() const {
  return size;
}

bool CustomDataObject::GetDataHere(void *buf) const
{
  memcpy(buf, buffer, size);
  return true;
}

bool CustomDataObject::SetData(size_t len, const void* buf){
  size = len;
  buffer = buf;
  return true;
}

extern "C" {

  struct Size {
    int x;
    int y;
  };

  void init_app(char* name, unsigned int width, unsigned int height) {
    setDPIAware();
    wxApp::SetInstance(new MyApp(name, wxSize(width, height)));
    int fake_argc = 0;
    char *fake_argv[1] = {NULL};
    wxEntryStart(fake_argc, fake_argv);
    wxTheApp->OnInit();

#ifdef __WINDOWS__
    // Adjust the window according to the display scale factor
    double scale = wxGetApp().frame->GetDPIScaleFactor();
    wxGetApp().frame->SetSize(width * scale, height * scale);
#else
    // OSX
    double scale = wxGetApp().frame->GetDPIScaleFactor();
    wxGetApp().frame->SetSize(width, height);
    if (wxGetApp().frame->GetPosition().y < 20.0) {
      wxGetApp().frame->Move(0.0, 20.0);
    }
#endif
  }

  void set_render(void (*render)()) {
    wxGetApp().frame->render = render;
  }

  void run_app() {
    wxTheApp->OnRun();
    wxTheApp->OnExit();
    wxEntryCleanup();
  }

#ifdef __APPLE__
  struct OSXHandle {
    void *ns_window;
    void *ns_view;
  };

  OSXHandle get_osx_raw_window_handle() {
    struct OSXHandle h = {NULL, NULL};
    h.ns_view = (void *) wxGetApp().frame->GetHandle();
    return h;
  }

#endif

  void close_app() {
    wxGetApp().frame->Close(true);
  }

#ifdef __WINDOWS__
  struct WindowsHandle {
    void *hwnd;
    void *hinstance;
  };

  WindowsHandle get_windows_raw_window_handle() {
    struct WindowsHandle h = {NULL, NULL};
    HWND hwnd = (HWND) wxGetApp().frame->GetHandle();
    h.hwnd = (void *) hwnd;
    h.hinstance = (void *) GetWindowLongPtr(hwnd, GWLP_HINSTANCE);
    return h;
  }
#endif

  // How many pixels in the canvas?
  Size get_display_size() {
#ifdef __APPLE__
    wxSize s =  wxGetApp().frame->GetClientSize();
    struct Size sz = {s.x, s.y};
    return sz;
#else
    struct wxSize s = wxGetApp().frame->GetClientSize();
    return Size { s.x, s.y };
#endif
  }

  // The logical window size
  Size get_client_size() {
#ifdef __APPLE__
    int status_bar_height;
    if (wxGetApp().frame->GetStatusBar()) {
      status_bar_height = wxGetApp().frame->GetStatusBar()->GetRect().height;
    } else {
      status_bar_height = 0;
    }
    float scale_factor = wxGetApp().frame->GetDPIScaleFactor();
    wxSize s =  wxGetApp().frame->GetClientSize();
    struct Size sz = {s.x * scale_factor, (s.y + status_bar_height) * scale_factor};
    return sz;
#else
    HWND hwnd = (HWND) wxGetApp().frame->GetHandle();
    RECT rect;
    GetClientRect(hwnd, &rect);
    float scale_factor = wxGetApp().frame->GetDPIScaleFactor();
    return Size {(int)((rect.right - rect.left) / scale_factor), (int)((rect.bottom - rect.top) / scale_factor)};
#endif
  }

  float get_scale_factor() {
    return wxGetApp().frame->GetDPIScaleFactor();
  }

  void refresh() {
    return wxGetApp().frame->Refresh();
  }

  void bind_canvas_events(void (*handle_events)(wxEvent &event)) {
    wxGetApp().frame->handle_events = handle_events;
    wxGetApp().frame->BindEvents();
  }

  int get_event_type(wxEvent &event) {
    return wxEventTypeToBridgeEventType(event.GetEventType());
  }

  int get_event_id(wxEvent &event) {
    return event.GetId();
  }

  int get_event_key(wxKeyEvent &event) {
    // std::cout << "Got code: " << event.GetKeyCode() << std::endl;
    return event.GetKeyCode();
  }

  wxChar get_event_char(wxKeyEvent &event) {
    return event.GetUnicodeKey();
  }

  int get_modifiers(wxKeyEvent &event) {
    return event.GetModifiers();
  }

  bool shift_down(wxKeyEvent &event) {
    return event.ShiftDown();
  }

  bool get_event_focused(wxActivateEvent &event) {
    return event.GetActive();
  }

  Size get_mouse_position(wxMouseEvent &event) {
#ifdef __APPLE__
    wxPoint p = event.GetPosition();
    float scale_factor = wxGetApp().frame->GetDPIScaleFactor();
    struct Size s = {p.x * scale_factor, p.y * scale_factor};
    return s;
#else
    wxPoint p = event.GetPosition();
    return Size {p.x, p.y};
#endif
  }

  int get_mouse_wheel_rotation(wxMouseEvent &event) {
    return event.GetWheelRotation();
  }

  int get_mouse_wheel_axis(wxMouseEvent &event) {
    return event.GetWheelAxis();
  }

  int get_mouse_wheel_delta(wxMouseEvent &event) {
    return event.GetWheelDelta();
  }

  void set_cursor(cursor c) {
    MyFrame *frame = wxGetApp().frame;
    frame->SetCursor(*frame->cursors[c]);
  }

  void create_status_bar() {
    wxGetApp().frame->CreateStatusBar();
  }

  void set_status_text(char *text) {
    if (wxGetApp().frame->GetStatusBar()) {
      wxGetApp().frame->SetStatusText(text);
    }
  }

  void put_string_on_clipboard(char *text) {
    if (wxTheClipboard->Open()) {
        wxTheClipboard->AddData(new wxTextDataObject(text));
        wxTheClipboard->Close();
    }
  }

  int get_clipboard_string_len() {
    if (wxTheClipboard->Open()) {
      wxTextDataObject data;
      if (wxTheClipboard->IsSupported(data.GetFormat())) {
        wxTheClipboard->GetData(data);
        int len = strlen((const char *) data.GetText().mb_str(wxConvUTF8));
        wxTheClipboard->Close();
        return len;
      } else {
        wxTheClipboard->Close();
        return -1;
      }
    } else {
      return -1;
    }
  }

  void get_string_from_clipboard(char *str) {
    if (wxTheClipboard->Open()) {
        wxTextDataObject data;
        wxTheClipboard->GetData(data);
        wxString txt = data.GetText();
        const char *src = (const char *) txt.mb_str(wxConvUTF8);
        strncpy(str, src, strlen(src));
        wxTheClipboard->Close();
    }
  }

  void put_buffer_on_clipboard(void *buf, int len) {
    if (wxTheClipboard->Open()) {
      CustomDataObject data;
      data.SetData(len, buf);
      wxTheClipboard->AddData(&data);
      wxTheClipboard->Close();
    }
  }

  int get_clipboard_buffer_len() {
    if (wxTheClipboard->Open()) {
      CustomDataObject data;
      if (wxTheClipboard->IsSupported(data.GetFormat())) {
        wxTheClipboard->GetData(data);
        int len = data.GetDataSize();
        wxTheClipboard->Close();
        return len;
      } else {
        wxTheClipboard->Close();
        return -1;
      }
    } else {
      return -1;
    }
  }

  void get_buffer_from_clipboard(char *buf) {
    if (wxTheClipboard->Open()) {
      CustomDataObject data;
      wxTheClipboard->GetData(data);
      data.GetDataHere(buf);
      wxTheClipboard->Close();
    }
  }

  void *create_menu() {
    return (void *) new wxMenu;
  }

  void insert_separator_to_menu(wxMenu* menu, size_t i) {
    if (i <= menu->GetMenuItemCount()) {
      menu->InsertSeparator(i);
    }
  }

  int insert_to_menu(wxMenu* menu, size_t i, char* str, char* help) {
    if (i <= menu->GetMenuItemCount()) {
      return menu->Insert(i, wxID_ANY, str, (help != NULL) ? (const wxChar *) help : wxEmptyString)->GetId();
    }
  }

  void insert_submenu(wxMenu* menu, size_t i, wxMenu* subMenu, char* str, char* help) {
    if (i <= menu->GetMenuItemCount()) {
      menu->Insert(i, wxID_ANY, str, subMenu, (help != NULL) ? (const wxChar *) help : wxEmptyString);
    }
  }

  void remove_from_menu(wxMenu* menu, size_t i) {
    if (i < menu->GetMenuItemCount()) {
      menu->Remove(i);
    }
  }

  void enable_menu_item(wxMenu* menu, size_t i, bool enable) {
    if (i < menu->GetMenuItemCount()) {
      menu->Enable(i, enable);
    }
  }

  void set_status_menu(wxMenu *menu) {
    wxGetApp().frame->PopupMenu(menu);
  }

  void delete_menu(wxMenu *menu) {
    delete menu;
  }

  void *create_menu_bar() {
    return (void *) new wxMenuBar;
  }

  void insert_to_menu_bar(wxMenuBar *menuBar, wxMenu* menu, size_t i, char* str) {
    if (i <= menuBar->GetMenuCount()) {
      menuBar->Insert(i, menu, str);
    }
  }

  void remove_from_menu_bar(wxMenuBar *menuBar, size_t i) {
    if (i < menuBar->GetMenuCount()) {
      menuBar->Remove(i);
    }
  }

  void set_menu_bar(wxMenuBar *menuBar) {
    wxGetApp().frame->SetMenuBar(menuBar);
  }

  void delete_menu_bar(wxMenuBar *menuBar) {
    if (wxGetApp().frame->GetMenuBar() == menuBar) {
      wxGetApp().frame->SetMenuBar(NULL);
    }
    delete menuBar;
  }
}
