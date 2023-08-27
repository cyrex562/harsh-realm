// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ImmoveableForm
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;
// usingSystem.ComponentModel;
// usingSystem.Runtime.InteropServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class ImmoveableForm : Form
  {
     const let mut HTCAPTION: i32 =  2;
     const let mut MF_BYCOMMAND: i32 =  0;
     const let mut MF_ENABLED: i32 =  0;
     const let mut MF_GRAYED: i32 =  1;
     const let mut MF_DISABLED: i32 =  2;
     const let mut SC_MOVE: i32 =  61456;
     const let mut WM_NCLBUTTONDOWN: i32 =  161;
     const let mut WM_SYSCOMMAND: i32 =  274;
     const let mut WM_INITMENUPOPUP: i32 =  279;
    pub bMoveable: bool;

    [DllImport("user32.dll", CharSet = CharSet.Ansi, SetLastError = true)]
     static extern EnableMenuItem: i32(IntPtr hMenu, uIDEnableItem: i32, uEnable: i32);

    pub ImmoveableForm() => this.bMoveable = true;

    [Description("Allows the form to be moved")]
    [Category("Behavior")]
    pub virtual bool Moveable
    {
      get => this.bMoveable;
      set
      {
        if (this.bMoveable == value)
          return;
        this.bMoveable = value;
      }
    }

    protected void WndProc( Message m)
    {
      if (m.Msg == 32)
        m = m;
      IntPtr num1;
      if (m.Msg == 279)
      {
        num1 = m.LParam;
        if (num1.ToInt32() / 65536 != 0)
        {
          let mut num2: i32 =  0;
          if (!this.Moveable)
            num2 = 3;
          ImmoveableForm.EnableMenuItem(m.WParam, 61456, 0 | num2);
        }
      }
      if (!this.Moveable)
      {
        if (m.Msg == 161)
        {
          num1 = m.WParam;
          if (num1.ToInt32() == 2)
            return;
        }
        if (m.Msg == 274)
        {
          num1 = m.WParam;
          if ((num1.ToInt32() & 65520) == 61456)
            return;
        }
      }
      base.WndProc( m);
    }
  }
}
