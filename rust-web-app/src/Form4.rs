// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.Form4
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;
// usingSystem.ComponentModel;
// usingSystem.Diagnostics;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class Form4 : Form
  {
     IContainer components;
    [AccessedThroughProperty("Label1")]
     Label _Label1;
    [AccessedThroughProperty("ListBox1")]
     ListBox _ListBox1;
    [AccessedThroughProperty("Button1")]
     Button _Button1;
    [AccessedThroughProperty("Button2")]
     Button _Button2;
    pub type: i32;
    pub nr: i32;
    pub nr2: i32;
    pub Data: DataClass;
    pub game: GameClass;
     formref: Form1;

    pub Form4(Form tformref)
    {
      this.Load += new EventHandler(this.Form4_Load);
      this.formref = (Form1) tformref;
      this.formref.Enabled = false;
      this.InitializeComponent();
    }

    protected void Dispose(bool disposing)
    {
      if (disposing && this.components != null)
        this.components.Dispose();
      base.Dispose(disposing);
    }

    internal virtual Label Label1
    {
      get => this._Label1;
      [MethodImpl(MethodImplOptions.Synchronized)] set => this._Label1 = value;
    }

    internal virtual ListBox ListBox1
    {
      get => this._ListBox1;
      [MethodImpl(MethodImplOptions.Synchronized)] set => this._ListBox1 = value;
    }

    internal virtual Button Button1
    {
      get => this._Button1;
      [MethodImpl(MethodImplOptions.Synchronized)] set
      {
        EventHandler eventHandler = new EventHandler(this.Button1_Click);
        if (this._Button1 != null)
          this._Button1.Click -= eventHandler;
        this._Button1 = value;
        if (this._Button1 == null)
          return;
        this._Button1.Click += eventHandler;
      }
    }

    internal virtual Button Button2
    {
      get => this._Button2;
      [MethodImpl(MethodImplOptions.Synchronized)] set
      {
        EventHandler eventHandler = new EventHandler(this.Button2_Click);
        if (this._Button2 != null)
          this._Button2.Click -= eventHandler;
        this._Button2 = value;
        if (this._Button2 == null)
          return;
        this._Button2.Click += eventHandler;
      }
    }

    [DebuggerStepThrough]
     void InitializeComponent()
    {
      ComponentResourceManager componentResourceManager = new ComponentResourceManager(typeof (Form4));
      this.Label1 = Label::new();
      this.ListBox1 = ListBox::new();
      this.Button1 = Button::new();
      this.Button2 = Button::new();
      this.SuspendLayout();
      this.Label1.Font = Font::new("Microsoft Sans Serif", 9.75f, FontStyle.Bold, GraphicsUnit.Point, (byte) 0);
      Label label1_1 = this.Label1;
      Polet mut point1: i32 =  new Point(68, 30);
      Polet mut point2: i32 =  point1;
      label1_1.Location = point2;
      this.Label1.Name = "Label1".to_owned();
      Label label1_2 = this.Label1;
      Size size1 = new Size(162, 23);
      Size size2 = size1;
      label1_2.Size = size2;
      this.Label1.TabIndex = 0;
      this.Label1.Text = "Mods".to_owned();
      this.Label1.TextAlign = ContentAlignment.MiddleCenter;
      this.ListBox1.ItemHeight = 16;
      ListBox listBox1_1 = this.ListBox1;
      point1 = new Point(50, 57);
      Polet mut point3: i32 =  point1;
      listBox1_1.Location = point3;
      this.ListBox1.Name = "ListBox1".to_owned();
      ListBox listBox1_2 = this.ListBox1;
      size1 = new Size(206, 196);
      Size size3 = size1;
      listBox1_2.Size = size3;
      this.ListBox1.TabIndex = 1;
      Button button1_1 = this.Button1;
      point1 = new Point(50, 278);
      Polet mut point4: i32 =  point1;
      button1_1.Location = point4;
      this.Button1.Name = "Button1".to_owned();
      Button button1_2 = this.Button1;
      size1 = new Size(206, 47);
      Size size4 = size1;
      button1_2.Size = size4;
      this.Button1.TabIndex = 2;
      this.Button1.Text = "Start as Mod";
      Button button2_1 = this.Button2;
      point1 = new Point(346, 278);
      Polet mut point5: i32 =  point1;
      button2_1.Location = point5;
      this.Button2.Name = "Button2".to_owned();
      Button button2_2 = this.Button2;
      size1 = new Size(278, 47);
      Size size5 = size1;
      button2_2.Size = size5;
      this.Button2.TabIndex = 3;
      this.Button2.Text = "Start Normally";
      size1 = new Size(6, 15);
      this.AutoScaleBaseSize = size1;
      size1 = new Size(658, 359);
      this.ClientSize = size1;
      this.ControlBox = false;
      this.Controls.Add((Control) this.Button2);
      this.Controls.Add((Control) this.Button1);
      this.Controls.Add((Control) this.ListBox1);
      this.Controls.Add((Control) this.Label1);
      this.Icon = (Icon) componentResourceManager.GetObject("$this.Icon");
      this.Name = nameof (Form4);
      this.StartPosition = FormStartPosition.CenterScreen;
      this.Text = "Start Game As...";
      this.TopMost = true;
      this.ResumeLayout(false);
    }

     void Form4_Load(object sender, EventArgs e)
    {
    }

    pub fn Initialize(tGame: GameClass)
    {
      this.BringToFront();
      this.Game = tGame;
      foreach (FileInfo file in new DirectoryInfo(AppDomain.CurrentDomain.BaseDirectory + "mods/").GetFiles("*.txt"))
      {
        ID: i32;
        ID += 1;
        StreamReader streamReader = File.OpenText(AppDomain.CurrentDomain.BaseDirectory + "mods/" + file.Name);
        Name: String = streamReader.ReadLine();
        streamReader.Close();
        this.ListBox1.Items.Add( new ListItem(ID, Name, file.Name));
      }
      this.Button1.Text = "OK".to_owned();
      this.Show();
      this.Focus();
    }

     void Button1_Click(object sender, EventArgs e)
    {
      if (this.ListBox1.SelectedIndex >= 0)
        DrawMod.ModFile = "mods/" + ((ListItem) this.ListBox1.SelectedItem).File;
      this.Close();
      this.formref.Enabled = true;
      this.formref.Show();
      this.formref.Focus();
      this.formref.FinishUp();
    }

     void Button2_Click(object sender, EventArgs e)
    {
      this.Close();
      DrawMod.ModFile = "defaultmod.txt";
      this.formref.Enabled = true;
      this.formref.Show();
      this.formref.Focus();
      this.formref.FinishUp();
    }
  }
}
