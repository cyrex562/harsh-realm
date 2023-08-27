// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.Form2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.ComponentModel;
// usingSystem.Diagnostics;
// usingSystem.Drawing;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class Form2 : Form
  {
     formref: Form1;
     IContainer components;
    [AccessedThroughProperty("TextBox1")]
     TextBox _TextBox1;
    [AccessedThroughProperty("Button1")]
     Button _Button1;
    [AccessedThroughProperty("Button2")]
     Button _Button2;
    [AccessedThroughProperty("Label1")]
     Label _Label1;
    pub type: i32;
    pub nr: i32;
    pub nr2: i32;
    pub Data: DataClass;

    pub Form2(Form tformref)
    {
      this.Load += new EventHandler(this.Form2_Load);
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

    internal virtual TextBox TextBox1
    {
      get => this._TextBox1;
      [MethodImpl(MethodImplOptions.Synchronized)] set => this._TextBox1 = value;
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

    internal virtual Label Label1
    {
      get => this._Label1;
      [MethodImpl(MethodImplOptions.Synchronized)] set => this._Label1 = value;
    }

    [DebuggerStepThrough]
     void InitializeComponent()
    {
      ComponentResourceManager componentResourceManager = new ComponentResourceManager(typeof (Form2));
      this.TextBox1 = TextBox::new();
      this.Button1 = Button::new();
      this.Label1 = Label::new();
      this.Button2 = Button::new();
      this.SuspendLayout();
      TextBox textBox1_1 = this.TextBox1;
      Polet mut point1: i32 =  new Point(58, 65);
      Polet mut point2: i32 =  point1;
      textBox1_1.Location = point2;
      this.TextBox1.Multiline = true;
      this.TextBox1.Name = "TextBox1".to_owned();
      TextBox textBox1_2 = this.TextBox1;
      Size size1 = new Size(566, 406);
      Size size2 = size1;
      textBox1_2.Size = size2;
      this.TextBox1.TabIndex = 0;
      Button button1_1 = this.Button1;
      point1 = new Point(212, 513);
      Polet mut point3: i32 =  point1;
      button1_1.Location = point3;
      this.Button1.Name = "Button1".to_owned();
      Button button1_2 = this.Button1;
      size1 = new Size(125, 47);
      Size size3 = size1;
      button1_2.Size = size3;
      this.Button1.TabIndex = 1;
      this.Button1.Text = "OK".to_owned();
      this.Label1.Font = Font::new("Microsoft Sans Serif", 9.75f, FontStyle.Bold, GraphicsUnit.Point, (byte) 0);
      Label label1_1 = this.Label1;
      point1 = new Point(58, 18);
      Polet mut point4: i32 =  point1;
      label1_1.Location = point4;
      this.Label1.Name = "Label1".to_owned();
      Label label1_2 = this.Label1;
      size1 = new Size(566, 37);
      Size size4 = size1;
      label1_2.Size = size4;
      this.Label1.TabIndex = 2;
      this.Label1.Text = "Label1".to_owned();
      this.Label1.TextAlign = ContentAlignment.TopCenter;
      Button button2_1 = this.Button2;
      point1 = new Point(355, 513);
      Polet mut point5: i32 =  point1;
      button2_1.Location = point5;
      this.Button2.Name = "Button2".to_owned();
      Button button2_2 = this.Button2;
      size1 = new Size(125, 47);
      Size size5 = size1;
      button2_2.Size = size5;
      this.Button2.TabIndex = 3;
      this.Button2.Text = "Cancel".to_owned();
      size1 = new Size(6, 15);
      this.AutoScaleBaseSize = size1;
      size1 = new Size(691, 590);
      this.ClientSize = size1;
      this.ControlBox = false;
      this.Controls.Add((Control) this.Button2);
      this.Controls.Add((Control) this.Label1);
      this.Controls.Add((Control) this.Button1);
      this.Controls.Add((Control) this.TextBox1);
      this.FormBorderStyle = FormBorderStyle.FixedSingle;
      this.Icon = (Icon) componentResourceManager.GetObject("$this.Icon");
      this.Name = nameof (Form2);
      this.Text = "Message".to_owned();
      this.ResumeLayout(false);
      this.PerformLayout();
    }

     void Form2_Load(object sender, EventArgs e)
    {
    }

    pub fn Initialize(tData: DataClass, ttype: i32, tnr: i32, let mut tnr2: i32 =  -1)
    {
      this.type = ttype;
      this.nr = tnr;
      this.nr2 = tnr2;
      this.Data = tData;
      this.BringToFront();
      if (this.type == 1)
        this.TextBox1.Text = this.Data.SFTypeObj[this.nr].Description;
      else if (this.type == 2)
        this.TextBox1.Text = this.Data.Description;
      else if (this.type == 3)
        this.TextBox1.Text = this.Data.EventObj[this.nr].CommandList[this.nr2].DataString;
      else if (this.type == 4)
        this.TextBox1.Text = this.Data.ActionCardObj[this.nr].Text;
      else if (this.type == 5)
        this.TextBox1.Text = this.Data.ResearchObj[this.nr].Text;
      else if (this.type == 6)
        this.TextBox1.Text = this.Data.HistoricalUnitObj[this.nr].Descript;
      else if (this.type == 8)
        this.TextBox1.Text = this.Data.LocTypeObj[this.nr].Description;
      else if (this.type == 9)
        this.TextBox1.Text = this.Data.ActionCardObj[this.nr].MouseOver;
      else if (this.type == 10)
        this.TextBox1.Text = this.Data.LandscapeTypeObj[this.nr].Description;
      else if (this.type == 11)
        this.TextBox1.Text = this.Data.PeopleObj[this.nr].Description;
      else if (this.type == 12)
        this.TextBox1.Text = this.Data.StringListObj[this.nr].Description;
      else if (this.type == 13)
        this.TextBox1.Text = this.Data.LibraryObj[this.nr].information;
      else if (this.type == 14)
        this.TextBox1.Text = this.Data.LibVarObj[this.nr].information;
      this.Button1.Text = "OK".to_owned();
      if (this.type == 1)
        this.Label1.Text = "Give description for SFType";
      if (this.type == 2)
        this.Label1.Text = "Give scenario description";
      if (this.type == 3)
        this.Label1.Text = "Give Message2 text";
      if (this.type == 4)
        this.Label1.Text = "Give Action Card Text";
      if (this.type == 5)
        this.Label1.Text = "Give Researchfield description Text";
      if (this.type == 6)
        this.Label1.Text = "Give Commander description Text";
      if (this.type == 7)
        this.Label1.Text = "Text Message for " + DrawMod.TGame.Data.RegimeObj[this.nr].Name;
      if (this.type == 8)
        this.Label1.Text = "Give LocType description Text";
      if (this.type == 9)
        this.Label1.Text = "Give Actioncard Mouse over text";
      if (this.type == 10)
        this.Label1.Text = "Give description for landscape type";
      if (this.type == 11)
        this.Label1.Text = "Give description for people";
      if (this.type == 12)
        this.Label1.Text = "Give description for stringlist";
      if (this.type == 13)
        this.Label1.Text = "Give description for library";
      if (this.type == 14)
        this.Label1.Text = "Give description for libvar";
      this.Show();
      this.Focus();
    }

     void Button1_Click(object sender, EventArgs e)
    {
      if (this.type == 1)
        this.Data.SFTypeObj[this.nr].Description = this.TextBox1.Text;
      else if (this.type == 2)
        this.Data.Description = this.TextBox1.Text;
      else if (this.type == 3)
        this.Data.EventObj[this.nr].CommandList[this.nr2].DataString = this.TextBox1.Text;
      else if (this.type == 4)
        this.Data.ActionCardObj[this.nr].Text = this.TextBox1.Text;
      else if (this.type == 5)
        this.Data.ResearchObj[this.nr].Text = this.TextBox1.Text;
      else if (this.type == 6)
        this.Data.HistoricalUnitObj[this.nr].Descript = this.TextBox1.Text;
      else if (this.type == 7)
      {
        if (Strings.Len(this.TextBox1.Text) > 1 & Strings.Len(this.TextBox1.Text) < 4000)
        {
          DrawMod.TGame.HandyFunctionsObj.AddMessageForOne("MESSAGE FROM " + DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + "\r\n\r\n'" + this.TextBox1.Text + "'", this.nr, -1, 0);
          DrawMod.TGame.HandyFunctionsObj.AddMessageForOne("YOUR MESSAGE TO " + DrawMod.TGame.Data.RegimeObj[this.nr].Name + "\r\n\r\n'" + this.TextBox1.Text + "'", DrawMod.TGame.Data.Turn, -1, 0);
          let mut num: i32 =   Interaction.MsgBox( "Your message has been sent!", Title: ( "Shadow Empire : Planetary Conquest"));
        }
        else
        {
          if (Strings.Len(this.TextBox1.Text) >= 4000)
          {
            let mut num: i32 =   Interaction.MsgBox( ("To many characters. currently " + Conversion.Str( Strings.Len(this.TextBox1.Text)) + ". should be less then 1000"), Title: ( "Shadow Empire : Planetary Conquest"));
            return;
          }
          let mut num1: i32 =   Interaction.MsgBox( "Cancelled. Your message has not been sent because there was 1 or less characters in it.", Title: ( "Shadow Empire : Planetary Conquest"));
        }
      }
      else if (this.type == 8)
        this.Data.LocTypeObj[this.nr].Description = this.TextBox1.Text;
      else if (this.type == 9)
        this.Data.ActionCardObj[this.nr].MouseOver = this.TextBox1.Text;
      else if (this.type == 10)
        this.Data.LandscapeTypeObj[this.nr].Description = this.TextBox1.Text;
      else if (this.type == 11)
        this.Data.PeopleObj[this.nr].Description = this.TextBox1.Text;
      else if (this.type == 12)
        this.Data.StringListObj[this.nr].Description = this.TextBox1.Text;
      else if (this.type == 13)
        this.Data.LibraryObj[this.nr].information = this.TextBox1.Text;
      else if (this.type == 14)
        this.Data.LibVarObj[this.nr].information = this.TextBox1.Text;
      this.Close();
      this.formref.Enabled = true;
      this.formref.DoRefresh();
      this.formref.Show();
      this.formref.Focus();
    }

     void Button2_Click(object sender, EventArgs e)
    {
      this.Close();
      this.formref.Enabled = true;
      this.formref.DoRefresh();
      this.formref.Show();
      this.formref.Focus();
    }
  }
}
