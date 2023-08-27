// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.Form3
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Collections;
// usingSystem.ComponentModel;
// usingSystem.Diagnostics;
// usingSystem.Drawing;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class Form3 : Form
  {
     formref: Form1;
     WindowClass WindowRef;
     IContainer components;
    [AccessedThroughProperty("Label1")]
     Label _Label1;
    [AccessedThroughProperty("ListBox1")]
     ListBox _ListBox1;
    [AccessedThroughProperty("Button1")]
     Button _Button1;
    [AccessedThroughProperty("Button3")]
     Button _Button3;
    [AccessedThroughProperty("Button2")]
     Button _Button2;
    pub type: i32;
    pub nr: i32;
    pub nr2: i32;
    pub nr3: i32;
    pub Data: DataClass;
    pub game: GameClass;

    pub Form3(Form tformref)
    {
      this.Load += new EventHandler(this.Form3_Load);
      this.formref = (Form1) tformref;
      this.formref.Enabled = false;
      this.InitializeComponent();
    }

    pub Form3(Form tformref, WindowClass tWindowRef)
    {
      this.Load += new EventHandler(this.Form3_Load);
      this.WindowRef = tWindowRef;
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

    internal virtual Button Button3
    {
      get => this._Button3;
      [MethodImpl(MethodImplOptions.Synchronized)] set
      {
        EventHandler eventHandler = new EventHandler(this.Button3_Click);
        if (this._Button3 != null)
          this._Button3.Click -= eventHandler;
        this._Button3 = value;
        if (this._Button3 == null)
          return;
        this._Button3.Click += eventHandler;
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
      ComponentResourceManager componentResourceManager = new ComponentResourceManager(typeof (Form3));
      this.Label1 = Label::new();
      this.ListBox1 = ListBox::new();
      this.Button1 = Button::new();
      this.Button2 = Button::new();
      this.Button3 = Button::new();
      this.SuspendLayout();
      this.Label1.Font = Font::new("Microsoft Sans Serif", 9.75f, FontStyle.Bold, GraphicsUnit.Point, (byte) 0);
      Label label1_1 = this.Label1;
      Polet mut point1: i32 =  new Point(58, 9);
      Polet mut point2: i32 =  point1;
      label1_1.Location = point2;
      this.Label1.Name = "Label1".to_owned();
      Label label1_2 = this.Label1;
      Size size1 = new Size(537, 28);
      Size size2 = size1;
      label1_2.Size = size2;
      this.Label1.TabIndex = 0;
      this.Label1.Text = "Label1".to_owned();
      this.Label1.TextAlign = ContentAlignment.MiddleCenter;
      this.ListBox1.ItemHeight = 16;
      ListBox listBox1_1 = this.ListBox1;
      point1 = new Point(86, 65);
      Polet mut point3: i32 =  point1;
      listBox1_1.Location = point3;
      this.ListBox1.Name = "ListBox1".to_owned();
      ListBox listBox1_2 = this.ListBox1;
      size1 = new Size(461, 340);
      Size size3 = size1;
      listBox1_2.Size = size3;
      this.ListBox1.TabIndex = 1;
      Button button1_1 = this.Button1;
      point1 = new Point(106, 489);
      Polet mut point4: i32 =  point1;
      button1_1.Location = point4;
      this.Button1.Name = "Button1".to_owned();
      Button button1_2 = this.Button1;
      size1 = new Size(153, 46);
      Size size4 = size1;
      button1_2.Size = size4;
      this.Button1.TabIndex = 2;
      this.Button1.Text = "OK".to_owned();
      Button button2_1 = this.Button2;
      point1 = new Point(384, 489);
      Polet mut point5: i32 =  point1;
      button2_1.Location = point5;
      this.Button2.Name = "Button2".to_owned();
      Button button2_2 = this.Button2;
      size1 = new Size(144, 46);
      Size size5 = size1;
      button2_2.Size = size5;
      this.Button2.TabIndex = 3;
      this.Button2.Text = "Cancel".to_owned();
      Button button3_1 = this.Button3;
      point1 = new Point(575, 352);
      Polet mut point6: i32 =  point1;
      button3_1.Location = point6;
      this.Button3.Name = "Button3".to_owned();
      Button button3_2 = this.Button3;
      size1 = new Size(117, 51);
      Size size6 = size1;
      button3_2.Size = size6;
      this.Button3.TabIndex = 4;
      this.Button3.Text = "Sort".to_owned();
      this.Button3.UseVisualStyleBackColor = true;
      size1 = new Size(6, 15);
      this.AutoScaleBaseSize = size1;
      size1 = new Size(720, 559);
      this.ClientSize = size1;
      this.ControlBox = false;
      this.Controls.Add((Control) this.Button3);
      this.Controls.Add((Control) this.Button2);
      this.Controls.Add((Control) this.Button1);
      this.Controls.Add((Control) this.ListBox1);
      this.Controls.Add((Control) this.Label1);
      this.FormBorderStyle = FormBorderStyle.FixedSingle;
      this.Icon = (Icon) componentResourceManager.GetObject("$this.Icon");
      this.Name = nameof (Form3);
      this.Text = "Select".to_owned();
      this.TopMost = true;
      this.ResumeLayout(false);
    }

     void Form3_Load(object sender, EventArgs e)
    {
    }

    pub void Initialize(
      tData: DataClass,
      ttype: i32,
      tnr: i32,
      let mut tnr2: i32 =  -1,
      let mut tGame: GameClass = null,
      let mut tnr3: i32 =  -1)
    {
      this.BringToFront();
      this.type = ttype;
      this.nr = tnr;
      this.nr2 = tnr2;
      this.nr3 = tnr3;
      this.Data = tData;
      this.Game = tGame;
      this.Button2.Visible = true;
      if (this.type == 130)
        this.Button2.Visible = false;
      num1: i32;
      if (this.type == 1)
      {
        let mut num2: i32 =  -1;
        let mut index: i32 =  100;
        do
        {
          num2 += 1;
          Name: String = Strings.Trim(Conversion.Str( num2)) + ") " + this.Data.TempString[index];
          this.ListBox1.Items.Add( new ListItem(num2, Name));
          index += 1;
        }
        while (index <= 199);
        if (this.nr2 > -1)
          this.ListBox1.SelectedIndex = this.nr2;
      }
      else if (this.type == 2)
      {
        let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
        for (let mut index: i32 =  0; index <= landscapeTypeCounter; index += 1)
        {
          if (!this.Data.LandscapeTypeObj[this.nr].CheckOverride(index))
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
      }
      else if (this.type == 3)
      {
        let mut peopleCounter: i32 =  this.Data.PeopleCounter;
        for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.PeopleObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 4)
      {
        let mut index: i32 =  0;
        do
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.TempString[index];
          this.ListBox1.Items.Add( new ListItem(index, Name));
          index += 1;
        }
        while (index <= 99);
      }
      else if (this.type == 5)
      {
        let mut num3: i32 =  -1;
        let mut index: i32 =  400;
        do
        {
          num3 += 1;
          Name: String = Strings.Trim(Conversion.Str( num3)) + ") " + this.Data.TempString[index];
          this.ListBox1.Items.Add( new ListItem(num3, Name));
          index += 1;
        }
        while (index <= 499);
      }
      else if (this.type == 6)
      {
        let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 7)
      {
        let mut peopleCounter: i32 =  this.Data.PeopleCounter;
        for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.PeopleObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 8)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None***"));
        let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
        for (let mut index: i32 =  0; index <= landscapeTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 9)
      {
        let mut basicSpriteCounter: i32 =  this.Data.LandscapeTypeObj[this.Data.LocTypeObj[this.nr].OverdrawLTNr].BasicSpriteCounter;
        for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[this.Data.LocTypeObj[this.nr].OverdrawLTNr].BasicSpriteFileName[index];
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 10)
      {
        this.ListBox1.Items.Add( new ListItem(-2, "***No Morph***"));
        this.ListBox1.Items.Add( new ListItem(-1, "***No Destruct***"));
        let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
        for (let mut index: i32 =  0; index <= landscapeTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 11)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None***"));
        let mut basicSpriteCounter: i32 =  this.Data.LandscapeTypeObj[this.Data.LocTypeObj[this.nr].OnDestructLT].BasicSpriteCounter;
        for (let mut index: i32 =  0; index <= basicSpriteCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[this.Data.LocTypeObj[this.nr].OnDestructLT].BasicSpriteFileName[index];
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 12)
      {
        let mut num4: i32 =  -1;
        let mut index: i32 =  500;
        do
        {
          num4 += 1;
          Name: String = Strings.Trim(Conversion.Str( num4)) + ") " + this.Data.TempString[index];
          this.ListBox1.Items.Add( new ListItem(num4, Name));
          index += 1;
        }
        while (index <= 599);
      }
      else if (this.type == 13)
      {
        let mut num5: i32 =  -1;
        let mut index: i32 =  200;
        do
        {
          num5 += 1;
          Name: String = Strings.Trim(Conversion.Str( num5)) + ") " + this.Data.TempString[index];
          this.ListBox1.Items.Add( new ListItem(num5, Name));
          index += 1;
        }
        while (index <= 299);
      }
      else if (this.type == 14 | this.type == 15)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None***"));
        let mut regimeCounter: i32 =  this.Data.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.RegimeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 16)
      {
        let mut peopleCounter: i32 =  this.Data.PeopleCounter;
        for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.PeopleObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 17)
      {
        let mut num6: i32 =  -1;
        let mut index: i32 =  300;
        do
        {
          num6 += 1;
          Name: String = Strings.Trim(Conversion.Str( num6)) + ") " + this.Data.TempString[index];
          this.ListBox1.Items.Add( new ListItem(num6, Name));
          index += 1;
        }
        while (index <= 399);
      }
      else if (this.type == 18)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
        let mut researchCounter: i32 =  this.Data.ResearchCounter;
        for (let mut index: i32 =  0; index <= researchCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.ResearchObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 19)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
        let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 20)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None***"));
        let mut itemTypeCounter: i32 =  this.Data.ItemTypeCounter;
        for (let mut index: i32 =  0; index <= itemTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.ItemTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 21)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None needed***"));
        let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 22 | this.type == 23)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
        let mut researchCounter: i32 =  this.Data.ResearchCounter;
        for (let mut index: i32 =  0; index <= researchCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.ResearchObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 24)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
        let mut researchCounter: i32 =  this.Data.ResearchCounter;
        for (let mut index: i32 =  0; index <= researchCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.ResearchObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type == 25)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
        let mut index: i32 =  0;
        do
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.GameSlotName[index];
          this.ListBox1.Items.Add( new ListItem(index, Name));
          index += 1;
        }
        while (index <= 499);
      }
      else if (this.type == 26)
      {
        this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
        let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
        {
          Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
          this.ListBox1.Items.Add( new ListItem(index, Name));
        }
      }
      else if (this.type != 27)
      {
        if (this.type == 28)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut peopleCounter: i32 =  this.Data.PeopleCounter;
          for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.PeopleObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 29)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut regimeCounter: i32 =  this.Data.RegimeCounter;
          for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.RegimeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 30)
        {
          let mut num7: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut index: i32 =  200;
          do
          {
            num7 += 1;
            Name: String = Strings.Trim(Conversion.Str( num7)) + ") " + this.Data.TempString[index];
            this.ListBox1.Items.Add( new ListItem(num7, Name));
            index += 1;
          }
          while (index <= 299);
        }
        else if (this.type == 31)
        {
          let mut num8: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num8 += 1;
            Name: String = Strings.Trim(Conversion.Str( num8)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num8, Name));
          }
        }
        else if (this.type == 32)
        {
          let mut num9: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut num10: i32 =  0;
          do
          {
            num9 += 1;
            Name: String = Strings.Trim(Conversion.Str( num9)) + ") " + this.Data.TempString[800 + num10];
            this.ListBox1.Items.Add( new ListItem(num9, Name));
            num10 += 1;
          }
          while (num10 <= 99);
        }
        else if (this.type == 33)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut unitCounter: i32 =  this.Data.UnitCounter;
          for (let mut ID: i32 =  0; ID <= unitCounter; ID += 1)
          {
            if (this.Data.UnitObj[ID].Regime == this.Data.Turn && this.Data.UnitObj[ID].PreDef == -1 && this.Data.UnitObj[ID].X > -1 && this.Data.UnitObj[ID].IsHQ)
              this.ListBox1.Items.Add( new ListItem(ID, this.Data.UnitObj[ID].Name + " (" + Strings.Trim(Conversion.Str( this.Data.UnitObj[ID].X)) + "," + Strings.Trim(Conversion.Str( this.Data.UnitObj[ID].Y)) + ")"));
          }
        }
        else if (this.type == 34)
        {
          let mut num11: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
          {
            num11 += 1;
            Name: String = Strings.Trim(Conversion.Str( num11)) + ") " + this.Data.SFTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num11, Name));
          }
        }
        else if (this.type == 35)
        {
          let mut num12: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num12 += 1;
            Name: String = Strings.Trim(Conversion.Str( num12)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num12, Name));
          }
        }
        else if (this.type == 36)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          this.ListBox1.Items.Add( new ListItem(1, "1) Independent Unit"));
          this.ListBox1.Items.Add( new ListItem(2, "2) Division"));
          this.ListBox1.Items.Add( new ListItem(5, "5) Corps"));
          this.ListBox1.Items.Add( new ListItem(6, "6) Army"));
          this.ListBox1.Items.Add( new ListItem(7, "7) Armygroup"));
          this.ListBox1.Items.Add( new ListItem(8, "8) Supreme HQ"));
        }
        else if (this.type == 37 | this.type == 46 | this.type == 47)
        {
          let mut Number: i32 =  -1;
          int[] numArray1 = new int[this.Data.HistoricalUnitCounter + 1];
          let mut unitCounter: i32 =  this.Data.UnitCounter;
          for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
          {
            if (this.Data.UnitObj[index1].Historical > -1)
            {
              int[] numArray2 = numArray1;
              int[] numArray3 = numArray2;
              let mut historical: i32 =  this.Data.UnitObj[index1].Historical;
              let mut index2: i32 =  historical;
              let mut num13: i32 =  numArray2[historical] + 1;
              numArray3[index2] = num13;
            }
          }
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut ID: i32 =  0; ID <= historicalUnitCounter; ID += 1)
          {
            let mut num14: i32 =  0;
            if (this.type == 47 | this.type == 46)
              num14 = 1;
            else if (this.Data.UnitObj[tnr].Regime == this.Data.HistoricalUnitObj[ID].TempRegime & this.type == 37 | this.type == 47)
              num14 = 1;
            if (this.type == 46 && this.Data.HistoricalUnitObj[this.nr].TempRegime == this.Data.HistoricalUnitObj[ID].TempRegime)
              num14 = 1;
            if (num14 == 1)
            {
              let mut num15: i32 =  0;
              if (this.type == 47 | this.type == 46)
                num15 = 1;
              else if (this.Data.UnitObj[tnr].IsHQ & (this.Data.HistoricalUnitObj[ID].Type >= 5 | this.Data.HistoricalUnitObj[ID].Type == -1) | !this.Data.UnitObj[tnr].IsHQ & this.Data.HistoricalUnitObj[ID].Type < 5 | this.type >= 46)
                num15 = 1;
              if (num15 == 1 && this.type == 37 | this.Data.HistoricalUnitObj[ID].Model)
              {
                Number += 1;
                str: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[ID].Name + "(";
                if (this.Data.HistoricalUnitObj[ID].Type == 1)
                  str += "Ind".to_owned();
                if (this.Data.HistoricalUnitObj[ID].Type == 2)
                  str += "Div".to_owned();
                if (this.Data.HistoricalUnitObj[ID].Type == 5)
                  str += "Corps".to_owned();
                if (this.Data.HistoricalUnitObj[ID].Type == 6)
                  str += "Army".to_owned();
                if (this.Data.HistoricalUnitObj[ID].Type == 7)
                  str += "Armygroup".to_owned();
                if (this.Data.HistoricalUnitObj[ID].Type == 8)
                  str += "High Command";
                Name: String = str + ")" + ", units = " + Conversion.Str( numArray1[ID]);
                this.ListBox1.Items.Add( new ListItem(ID, Name));
              }
            }
          }
        }
        else if (this.type == 38 | this.type == 139)
        {
          let mut num16: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut actionCardCounter: i32 =  this.Data.ActionCardCounter;
          for (let mut index: i32 =  0; index <= actionCardCounter; index += 1)
          {
            num16 += 1;
            Name: String = Strings.Trim(Conversion.Str( num16)) + ") " + this.Data.ActionCardObj[index].Title;
            this.ListBox1.Items.Add( new ListItem(num16, Name));
          }
        }
        else if (this.type == 39)
        {
          let mut num17: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num17 += 1;
            Name: String = Strings.Trim(Conversion.Str( num17)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num17, Name));
          }
        }
        else if (this.type == 40)
        {
          let mut num18: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut actionCardCounter: i32 =  this.Data.ActionCardCounter;
          for (let mut index: i32 =  0; index <= actionCardCounter; index += 1)
          {
            num18 += 1;
            Name: String = Strings.Trim(Conversion.Str( num18)) + ") " + this.Data.ActionCardObj[index].Title;
            this.ListBox1.Items.Add( new ListItem(num18, Name));
          }
        }
        else if (this.type == 41)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          if (this.Game.Data.UnitObj[this.Game.EditObj.OrderUnit].TempCategory == 1)
          {
            let mut counter: i32 =  this.Game.NewAIObj.MarkerList.Counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (this.Game.NewAIObj.MarkerList.Data1[index] == this.Game.SelectX & this.Game.NewAIObj.MarkerList.Data2[index] == this.Game.SelectY)
              {
                Name: String = Strings.Trim(Conversion.Str( index)) + ") Normal Target = " + Conversion.Str( this.Game.NewAIObj.MarkerList.Data3[index]) + "," + Conversion.Str( this.Game.NewAIObj.MarkerList.Data4[index]);
                this.ListBox1.Items.Add( new ListItem(index, Name));
              }
            }
          }
          if (this.Game.Data.UnitObj[this.Game.EditObj.OrderUnit].TempCategory == 2)
          {
            let mut counter: i32 =  this.Game.NewAIObj.ArtMarkerList.Counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (this.Game.NewAIObj.ArtMarkerList.Data1[index] == this.Game.SelectX & this.Game.NewAIObj.ArtMarkerList.Data2[index] == this.Game.SelectY)
              {
                Name: String = Strings.Trim(Conversion.Str( index)) + ") Artillery Target = " + Conversion.Str( this.Game.NewAIObj.ArtMarkerList.Data3[index]) + "," + Conversion.Str( this.Game.NewAIObj.ArtMarkerList.Data4[index]);
                this.ListBox1.Items.Add( new ListItem(index, Name));
              }
            }
          }
          if (this.Game.Data.UnitObj[this.Game.EditObj.OrderUnit].TempCategory == 3)
          {
            let mut counter: i32 =  this.Game.NewAIObj.AirMarkerList.Counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              if (this.Game.NewAIObj.AirMarkerList.Data1[index] == this.Game.SelectX & this.Game.NewAIObj.AirMarkerList.Data2[index] == this.Game.SelectY)
              {
                Name: String = Strings.Trim(Conversion.Str( index)) + ") Air Target = " + Conversion.Str( this.Game.NewAIObj.AirMarkerList.Data3[index]) + "," + Conversion.Str( this.Game.NewAIObj.AirMarkerList.Data4[index]);
                this.ListBox1.Items.Add( new ListItem(index, Name));
              }
            }
          }
          if (this.Game.Data.UnitObj[this.Game.EditObj.OrderUnit].TempCategory == 4)
          {
            if (this.Game.NewAIObj.EngineerMarkerList.Counter > -1)
            {
              let mut counter: i32 =  this.Game.NewAIObj.EngineerMarkerList.Counter;
              for (let mut index: i32 =  0; index <= counter; index += 1)
              {
                if (this.Game.NewAIObj.EngineerMarkerList.Data1[index] == this.Game.SelectX & this.Game.NewAIObj.EngineerMarkerList.Data2[index] == this.Game.SelectY)
                {
                  Name: String = Strings.Trim(Conversion.Str( index)) + ") Engineer Target = " + Conversion.Str( this.Game.NewAIObj.EngineerMarkerList.Data3[index]) + "," + Conversion.Str( this.Game.NewAIObj.EngineerMarkerList.Data4[index]);
                  this.ListBox1.Items.Add( new ListItem(index, Name));
                }
              }
            }
            else
            {
              let mut counter: i32 =  this.Game.NewAIObj.MarkerList.Counter;
              for (let mut index: i32 =  0; index <= counter; index += 1)
              {
                if (this.Game.NewAIObj.MarkerList.Data1[index] == this.Game.SelectX & this.Game.NewAIObj.MarkerList.Data2[index] == this.Game.SelectY)
                {
                  Name: String = Strings.Trim(Conversion.Str( index)) + ") Normal Target = " + Conversion.Str( this.Game.NewAIObj.MarkerList.Data3[index]) + "," + Conversion.Str( this.Game.NewAIObj.MarkerList.Data4[index]);
                  this.ListBox1.Items.Add( new ListItem(index, Name));
                }
              }
            }
          }
        }
        else if (this.type == 42)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None needed***"));
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 43)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None needed***"));
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 44)
        {
          let mut num19: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***Default***"));
          let mut index: i32 =  0;
          do
          {
            num19 += 1;
            Name: String = Strings.Trim(Conversion.Str( num19)) + ") " + this.Data.TempString[index];
            this.ListBox1.Items.Add( new ListItem(num19, Name));
            index += 1;
          }
          while (index <= 99);
        }
        else if (this.type == 45 | this.type == 136)
        {
          let mut num20: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None/Default***"));
          let mut unitCounter: i32 =  this.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (this.Data.UnitObj[index].PreDef > -1)
            {
              num20 += 1;
              Name: String = Strings.Trim(Conversion.Str( this.Data.UnitObj[index].PreDef)) + ") " + this.Data.UnitObj[index].Name;
              this.ListBox1.Items.Add( new ListItem(this.Data.UnitObj[index].PreDef, Name));
            }
          }
        }
        else if (this.type == 48)
        {
          let mut num21: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***Default***"));
          let mut index: i32 =  0;
          do
          {
            if (this.Data.HistoricalUnitObj[this.nr2].SubParts[index] > -1 | this.Data.HistoricalUnitObj[this.nr2].Designation[index] > -1)
            {
              num21 += 1;
              Name: String;
              if (this.Data.HistoricalUnitObj[this.nr2].SubParts[index] > -1)
                Name = Conversion.Str( index) + ") " + this.Data.UnitObj[this.Data.HistoricalUnitObj[this.nr2].SubParts[index]].Name + ", " + Conversion.Str( this.Data.HistoricalUnitObj[this.nr2].Designation[index]);
              else
                Name = Conversion.Str( index) + ") " + Conversion.Str( this.Data.HistoricalUnitObj[this.nr2].Designation[index]);
              this.ListBox1.Items.Add( new ListItem(index, Name));
            }
            index += 1;
          }
          while (index <= 9);
        }
        else if (this.type == 49)
        {
          let mut num22: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** None / No overrule ***"));
          this.ListBox1.Items.Add( new ListItem(-2, "*** People of the Regime Producing ***"));
          let mut peopleCounter: i32 =  this.Data.PeopleCounter;
          for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
          {
            num22 += 1;
            Name: String = Conversion.Str( index) + ") " + this.Data.PeopleObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 50)
        {
          let mut num23: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***Default***"));
          let mut index: i32 =  0;
          do
          {
            num23 += 1;
            Name: String = Strings.Trim(Conversion.Str( num23)) + ") " + this.Data.TempString[index];
            this.ListBox1.Items.Add( new ListItem(num23, Name));
            index += 1;
          }
          while (index <= 99);
        }
        else if (this.type == 51)
        {
          let mut num24: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***Default***"));
          let mut reinfCounter: i32 =  this.Data.ReinfCounter;
          for (let mut index: i32 =  0; index <= reinfCounter; index += 1)
          {
            num24 += 1;
            Name: String = Strings.Trim(Conversion.Str( num24)) + ") " + this.Data.ReinfName[index];
            this.ListBox1.Items.Add( new ListItem(num24, Name));
          }
        }
        else if (this.type == 53 | this.type == 54)
        {
          let mut num25: i32 =  -1;
          let mut regimeCounter: i32 =  this.Data.RegimeCounter;
          for (let mut ID: i32 =  0; ID <= regimeCounter; ID += 1)
          {
            num25 += 1;
            if (this.Data.Turn != ID && this.Data.RegimeObj[this.Data.Turn].RegimeRel[ID] == 2)
            {
              if (this.type == 53)
              {
                name: String = this.Data.RegimeObj[ID].Name;
                this.ListBox1.Items.Add( new ListItem(ID, name));
              }
              else if (this.type != 54 | !this.Data.RegimeObj[ID].ResField[this.nr])
              {
                name: String = this.Data.RegimeObj[ID].Name;
                this.ListBox1.Items.Add( new ListItem(ID, name));
              }
            }
          }
        }
        else if (this.type == 52)
        {
          let mut num26: i32 =  -1;
          let mut regimeCounter: i32 =  this.Data.RegimeCounter;
          for (let mut ID: i32 =  0; ID <= regimeCounter; ID += 1)
          {
            num26 += 1;
            if (this.Data.Turn != ID && this.Data.RegimeObj[this.Data.Turn].RegimeRel[ID] == 2 && this.Data.UnitObj[this.nr].Regime != ID)
            {
              name: String = this.Data.RegimeObj[ID].Name;
              this.ListBox1.Items.Add( new ListItem(ID, name));
            }
          }
          if (this.Data.RegimeObj[this.Data.UnitObj[this.nr].Regime].UberRegime == this.Data.Turn)
            this.ListBox1.Items.Add( new ListItem(this.Data.Turn, "*** Give to your self (as uber-regime) ***"));
        }
        else if (this.type == 55)
        {
          let mut num27: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None***"));
          let mut stringListCounter: i32 =  this.Data.StringListCounter;
          for (let mut index: i32 =  0; index <= stringListCounter; index += 1)
          {
            num27 += 1;
            Name: String = Conversion.Str( this.Data.StringListObj[index].ID) + ") " + this.Data.StringListObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(this.Data.StringListObj[index].ID, Name));
          }
        }
        else if (this.type == 56)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Format ***"));
          this.ListBox1.Items.Add( new ListItem(0, "UDS Management Tabs Format"));
        }
        else if (this.type == 57)
        {
          let mut num28: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No UberRegime ***"));
          let mut regimeCounter: i32 =  this.Data.RegimeCounter;
          for (let mut ID: i32 =  0; ID <= regimeCounter; ID += 1)
          {
            num28 += 1;
            name: String = this.Data.RegimeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, name));
          }
        }
        else if (this.type == 58)
        {
          let mut num29: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num29 += 1;
            Name: String = Strings.Trim(Conversion.Str( num29)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num29, Name));
          }
        }
        else if (this.type == 59)
        {
          num1 = -1;
          let mut unitCounter: i32 =  this.Data.UnitCounter;
          for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
          {
            if (tGame.Data.UnitObj[index3].Regime == tGame.Data.Turn && tGame.HandyFunctionsObj.HasUnitAirSF(index3))
            {
              if (tGame.HandyFunctionsObj.CanDoAirStrike(index3, Coordinate::new()
              {
                x = tGame.EditObj.TargetX,
                y = tGame.EditObj.TargetY
              }))
              {
                Name: String = this.Data.UnitObj[index3].Name;
                if (tGame.EditObj.TempUnitList.CheckIfPresent(index3))
                  Name = "SELECTED " + Name;
                if (tGame.Data.UnitObj[index3].HQ > -1)
                  Name = Name + ", " + tGame.Data.UnitObj[tGame.Data.UnitObj[index3].HQ].Name;
                let mut sfCount: i32 =  this.Game.Data.UnitObj[index3].SFCount;
                for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
                {
                  let mut sf: i32 =  tGame.Data.UnitObj[index3].SFList[index4];
                  let mut type: i32 =  tGame.Data.SFObj[sf].Type;
                  if (tGame.Data.SFTypeObj[type].Theater == 2)
                    Name = Name + ", " + Strings.Trim(Conversion.Str( (tGame.Data.SFObj[sf].Qty * tGame.Data.SFTypeObj[type].Ratio))) + "x " + tGame.Data.SFTypeObj[type].Name;
                }
                this.ListBox1.Items.Add( new ListItem(index3, Name));
              }
            }
          }
        }
        else if (this.type == 60)
        {
          num1 = -1;
          let mut unitCounter: i32 =  tGame.Data.UnitCounter;
          for (let mut ID: i32 =  0; ID <= unitCounter; ID += 1)
          {
            if (tGame.Data.UnitObj[ID].Regime == tGame.Data.Turn && tGame.Data.UnitObj[ID].IsHQ & (tGame.Data.UnitObj[ID].LandCap > 0 | tGame.Data.UnitObj[ID].AirCap > 0 | tGame.Data.UnitObj[ID].NavyCap > 0))
            {
              Name: String = this.Data.UnitObj[ID].Name + ", LandCap=" + Strings.Trim(Conversion.Str( this.Game.Data.UnitObj[ID].LandCap)) + ", NavyCap=" + Strings.Trim(Conversion.Str( this.Game.Data.UnitObj[ID].NavyCap)) + ", RailCap=" + Strings.Trim(Conversion.Str( this.Game.Data.UnitObj[ID].AirCap));
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 61)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
          for (let mut index: i32 =  0; index <= landscapeTypeCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 62)
        {
          let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
          for (let mut index: i32 =  0; index <= landscapeTypeCounter; index += 1)
          {
            if (!this.Data.LandscapeTypeObj[this.nr].CheckOverride2(index))
            {
              Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.LandscapeTypeObj[index].Name;
              this.ListBox1.Items.Add( new ListItem(index, Name));
            }
          }
        }
        else if (this.type == 63)
        {
          let mut num30: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num30 += 1;
            Name: String = Strings.Trim(Conversion.Str( num30)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num30, Name));
          }
        }
        else if (this.type == 64)
        {
          let mut num31: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num31 += 1;
            Name: String = Strings.Trim(Conversion.Str( num31)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num31, Name));
          }
        }
        else if (this.type == 65)
        {
          let mut num32: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None***"));
          let mut stringListCounter: i32 =  this.Data.StringListCounter;
          for (let mut index: i32 =  0; index <= stringListCounter; index += 1)
          {
            num32 += 1;
            Name: String = Conversion.Str( this.Data.StringListObj[index].ID) + ") " + this.Data.StringListObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(this.Data.StringListObj[index].ID, Name));
          }
        }
        else if (this.type == 66)
        {
          let mut num33: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num33 += 1;
            Name: String = Strings.Trim(Conversion.Str( num33)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num33, Name));
          }
        }
        else if (this.type == 67)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "*** All ***"));
          let mut itemTypeCounter: i32 =  this.Data.ItemTypeCounter;
          for (let mut index: i32 =  0; index <= itemTypeCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.ItemTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 68 | this.type == 69)
        {
          let mut num34: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** All ***"));
          let mut index: i32 =  400;
          do
          {
            num34 += 1;
            Name: String = Strings.Trim(Conversion.Str( num34)) + ") " + this.Data.TempString[index];
            this.ListBox1.Items.Add( new ListItem(num34, Name));
            index += 1;
          }
          while (index <= 499);
        }
        else if (this.type == 70)
        {
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.SFTypeObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 71 | this.type == 72)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut researchCounter: i32 =  this.Data.ResearchCounter;
          for (let mut index: i32 =  0; index <= researchCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.ResearchObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 73)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut index: i32 =  0;
          do
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.RegimeSlotName[index];
            this.ListBox1.Items.Add( new ListItem(index, Name));
            index += 1;
          }
          while (index <= 499);
        }
        else if (this.type == 74)
        {
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          ListClass listClass = ListClass::new();
          let mut num35: i32 =  -1;
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut tdata: i32 =  0; tdata <= sfTypeCounter; tdata += 1)
          {
            if (!this.Data.SFTypeObj[tdata].DontShowInList && this.Data.SFTypeObj[tdata].Name.Length > 1)
            {
              num35 += 1;
              name: String = this.Data.SFTypeObj[tdata].Name;
              listClass.add(name, tdata);
            }
          }
          listClass.Sort();
          let mut listCount: i32 =  listClass.ListCount;
          for (let mut index: i32 =  0; index <= listCount; index += 1)
            this.ListBox1.Items.Add( new ListItem(listClass.ListData[index], listClass.ListName[index]));
        }
        else if (this.type >= 75 & this.type <= 78 | this.type == 81)
        {
          let mut num36: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut eventCounter: i32 =  this.Data.EventCounter;
          for (let mut index: i32 =  0; index <= eventCounter; index += 1)
          {
            num36 += 1;
            Name: String = Strings.Trim(Conversion.Str( num36)) + ") " + this.Data.EventObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(num36, Name));
          }
        }
        else if (this.type == 79)
        {
          let mut num37: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***No Alteration***"));
          let mut modelVariantCounter: i32 =  this.Data.SFTypeObj[this.nr].ModelVariantCounter;
          for (let mut ID: i32 =  0; ID <= modelVariantCounter; ID += 1)
          {
            if (this.Data.SFTypeObj[this.nr].TempAlterationPossible[ID])
            {
              num37 += 1;
              Name: String = this.Data.SFTypeObj[this.nr].ModelVariantName[ID];
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 80)
        {
          let mut num38: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***No AutoProd ***"));
          let mut itemTypeCounter: i32 =  this.Data.ItemTypeCounter;
          for (let mut ID: i32 =  0; ID <= itemTypeCounter; ID += 1)
          {
            num38 += 1;
            name: String = this.Data.ItemTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, name));
          }
        }
        else if (this.type == 82)
        {
          num1 = -1;
          if (this.Game.EditObj.inSimpleEditor)
            this.ListBox1.Items.Add( new ListItem(-1, "***No HQ****"));
          let mut unitCounter: i32 =  this.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (this.Data.UnitObj[index].PreDef == -1 & this.Data.UnitObj[index].X > -1 && this.nr > -1 && this.nr != index & this.Game.Data.UnitObj[this.nr].HQ != index & this.Game.Data.UnitObj[index].IsHQ & (this.Game.Data.UnitObj[index].Regime == this.Game.Data.Turn | this.Game.Data.UnitObj[index].Regime == this.Game.Data.UnitObj[this.nr].Regime) && this.Game.HandyFunctionsObj.CanUnitBecomeHQfor(index, this.nr))
            {
              let mut num39: i32 =  0;
              if (this.Game.Data.UnitObj[this.nr].IsHQ)
                num39 = 1;
              if ( this.Game.Data.RuleVar[304] == 0.0 |  (this.Game.HandyFunctionsObj.HowmanyHQsAbove(index) + this.Game.HandyFunctionsObj.HowmanyHQsBelow(this.nr) + 1 + num39) <=  this.Game.Data.RuleVar[304])
              {
                name: String = this.Game.Data.UnitObj[index].Name;
                this.ListBox1.Items.Add( new ListItem(index, name));
              }
            }
          }
        }
        else if (this.type == 83 | this.type == 137)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***No ppl ***"));
          let mut peopleCounter: i32 =  this.Data.PeopleCounter;
          for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.PeopleObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 84 | this.type == 100 | this.type == 138)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** None ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          {
            if (this.Data.HistoricalUnitObj[index].Model & this.Data.HistoricalUnitObj[index].SubParts[0] > -1)
            {
              if (this.type == 100)
              {
                if (this.Game.Data.HistoricalUnitObj[index].TempRegime == this.nr)
                {
                  Name: String = Strings.Trim(Conversion.Str( index)) + ") MODEL " + this.Data.HistoricalUnitObj[index].Name;
                  this.ListBox1.Items.Add( new ListItem(index, Name, RealName: this.Data.HistoricalUnitObj[index].Name));
                }
              }
              else
              {
                Name: String = Strings.Trim(Conversion.Str( index)) + ") MODEL " + this.Data.HistoricalUnitObj[index].Name;
                this.ListBox1.Items.Add( new ListItem(index, Name, RealName: this.Data.HistoricalUnitObj[index].Name));
              }
            }
          }
        }
        else if (this.type == 85)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** None ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          {
            if (this.Data.HistoricalUnitObj[index].DeckCardCounter > -1)
            {
              Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.HistoricalUnitObj[index].Name;
              this.ListBox1.Items.Add( new ListItem(index, Name, RealName: this.Data.HistoricalUnitObj[index].Name));
            }
          }
        }
        else if (this.type == 86)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** None ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          {
            if (this.Data.HistoricalUnitObj[index].HisVarCount > -1)
            {
              Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.HistoricalUnitObj[index].Name;
              this.ListBox1.Items.Add( new ListItem(index, Name, RealName: this.Data.HistoricalUnitObj[index].Name));
            }
          }
        }
        else if (this.type == 87 | this.type == 101)
        {
          let mut Number: i32 =  -1;
          int[] numArray4 = new int[this.Data.HistoricalUnitCounter + 1];
          let mut unitCounter: i32 =  this.Data.UnitCounter;
          for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
          {
            if (this.Data.UnitObj[index5].Historical > -1)
            {
              int[] numArray5 = numArray4;
              int[] numArray6 = numArray5;
              let mut historical: i32 =  this.Data.UnitObj[index5].Historical;
              let mut index6: i32 =  historical;
              let mut num40: i32 =  numArray5[historical] + 1;
              numArray6[index6] = num40;
            }
          }
          let mut num41: i32 =  -1;
          if (this.Game.SelectX > -1 & this.Game.SelectY > -1)
            num41 = this.Game.Data.MapObj[0].HexObj[this.Game.SelectX, this.Game.SelectY].Regime;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut ID: i32 =  0; ID <= historicalUnitCounter; ID += 1)
          {
            let mut num42: i32 =  1;
            if (numArray4[ID] > 0)
              num42 = 0;
            if (num42 == 1 && !this.Data.HistoricalUnitObj[ID].Model && num41 == -1 | this.Data.HistoricalUnitObj[ID].TempRegime == num41 & !(this.type == 101 & this.Data.HistoricalUnitObj[ID].CommanderName.Length > 0))
            {
              Number += 1;
              str: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[ID].Name + "(";
              if (this.Data.HistoricalUnitObj[ID].Type == 1)
                str += "Ind".to_owned();
              if (this.Data.HistoricalUnitObj[ID].Type == 2)
                str += "Multi-Unit";
              if (this.Data.HistoricalUnitObj[ID].Type == 5)
                str += "Low HQ";
              if (this.Data.HistoricalUnitObj[ID].Type == 6)
                str += "Med HQ";
              if (this.Data.HistoricalUnitObj[ID].Type == 7)
                str += "High HQ";
              if (this.Data.HistoricalUnitObj[ID].Type == 8)
                str += "High Command";
              Name: String = str + ")" + ", units = " + Conversion.Str( numArray4[ID]);
              this.ListBox1.Items.Add( new ListItem(ID, Name, RealName: this.Data.HistoricalUnitObj[ID].Name));
            }
          }
        }
        else if (this.type == 88)
        {
          num1 = -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***No ppl ***"));
          let mut peopleCounter: i32 =  this.Data.PeopleCounter;
          for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
          {
            Name: String = Strings.Trim(Conversion.Str( index)) + ") " + this.Data.PeopleObj[index].Name;
            this.ListBox1.Items.Add( new ListItem(index, Name));
          }
        }
        else if (this.type == 89)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***Default***"));
          let mut reinfCounter: i32 =  this.Data.ReinfCounter;
          for (let mut ID: i32 =  0; ID <= reinfCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.ReinfName[ID];
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 90)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut libraryCounter: i32 =  this.Data.LibraryCounter;
          for (let mut ID: i32 =  0; ID <= libraryCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LibraryObj[ID].name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 91)
        {
          this.ListBox1.Items.Add( new ListItem(0, "Global/General"));
          this.ListBox1.Items.Add( new ListItem(1, "Landscape"));
          this.ListBox1.Items.Add( new ListItem(2, "Road"));
          this.ListBox1.Items.Add( new ListItem(3, "River"));
          this.ListBox1.Items.Add( new ListItem(4, "Hex"));
          this.ListBox1.Items.Add( new ListItem(5, "SFType"));
          this.ListBox1.Items.Add( new ListItem(6, "LocationType"));
          this.ListBox1.Items.Add( new ListItem(7, "HistoricalUnit"));
          this.ListBox1.Items.Add( new ListItem(8, "HistoricalUnitModel"));
          this.ListBox1.Items.Add( new ListItem(9, "Officer"));
          this.ListBox1.Items.Add( new ListItem(10, "People"));
          this.ListBox1.Items.Add( new ListItem(11, "Regime"));
        }
        else if (this.type == 92 | this.type == 105)
        {
          this.ListBox1.Items.Add( new ListItem(0, "Number"));
          this.ListBox1.Items.Add( new ListItem(1, "Text"));
          this.ListBox1.Items.Add( new ListItem(2, "RoadSlot"));
          this.ListBox1.Items.Add( new ListItem(3, "LandscapeSlot"));
          this.ListBox1.Items.Add( new ListItem(4, "RiverSlot"));
          this.ListBox1.Items.Add( new ListItem(5, "DateString"));
          this.ListBox1.Items.Add( new ListItem(6, "SFTypeSlot"));
          this.ListBox1.Items.Add( new ListItem(7, "HistoricalUnitSlot"));
          this.ListBox1.Items.Add( new ListItem(8, "HistoricalUnitModelSlot"));
          this.ListBox1.Items.Add( new ListItem(9, "OfficerSlot"));
          this.ListBox1.Items.Add( new ListItem(10, "PeopleSlot"));
          this.ListBox1.Items.Add( new ListItem(11, "RegimeSlot"));
          this.ListBox1.Items.Add( new ListItem(12, "Yes/No"));
          this.ListBox1.Items.Add( new ListItem(13, "LocationType"));
          this.ListBox1.Items.Add( new ListItem(14, "SmallGfx"));
          this.ListBox1.Items.Add( new ListItem(15, "EventPic"));
          this.ListBox1.Items.Add( new ListItem(16, "ActionCardSlot"));
        }
        else if (this.type == 93)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut libraryCounter: i32 =  this.Data.LibraryCounter;
          for (let mut ID: i32 =  0; ID <= libraryCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LibraryObj[ID].name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 94)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut libraryCounter: i32 =  this.Data.LibraryCounter;
          for (let mut ID: i32 =  0; ID <= libraryCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LibraryObj[ID].name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 95 | this.type == 96 | this.type == 97 | this.type == 107 | this.type == 108 | this.type == 109 | this.type == 129)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut libraryCounter: i32 =  this.Data.LibraryCounter;
          for (let mut ID: i32 =  0; ID <= libraryCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LibraryObj[ID].name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 98)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** Do NOT substitute ***"));
          let mut regimeCounter: i32 =  this.Data.RegimeCounter;
          for (let mut ID: i32 =  0; ID <= regimeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.RegimeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 99)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** Do NOT substitute ***"));
          let mut peopleCounter: i32 =  this.Data.PeopleCounter;
          for (let mut ID: i32 =  0; ID <= peopleCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.PeopleObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 102)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** No commander ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          {
            if (this.Game.Data.HistoricalUnitObj[index].TempRegime == this.nr & this.Game.Data.HistoricalUnitObj[index].CommanderName.Length > 0 & !this.Game.Data.HistoricalUnitObj[index].Pool & this.Game.HandyFunctionsObj.GetUnitByHistorical(index) == -1)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[index].CommanderName;
              this.ListBox1.Items.Add( new ListItem(index, Name));
            }
          }
        }
        else if (this.type == 103)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** No commander ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          {
            if (this.Game.Data.HistoricalUnitObj[index].TempRegime == this.nr & this.Game.Data.HistoricalUnitObj[index].CommanderName.Length > 0 & this.Game.Data.HistoricalUnitObj[index].Pool & this.Game.HandyFunctionsObj.GetUnitByHistorical(index) == -1)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[index].CommanderName;
              this.ListBox1.Items.Add( new ListItem(index, Name));
            }
          }
        }
        else if (this.type == 104)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***None ***"));
          let mut libraryCounter: i32 =  this.Data.LibraryCounter;
          for (let mut ID: i32 =  0; ID <= libraryCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LibraryObj[ID].name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 106 | this.type == 123)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Regime ***"));
          let mut regimeCounter: i32 =  this.Data.RegimeCounter;
          for (let mut ID: i32 =  0; ID <= regimeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.RegimeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 110 | this.type == 118)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No His Unit ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          {
            if ((this.type == 110 | !this.Data.HistoricalUnitObj[index].Model) & this.Data.HistoricalUnitObj[index].CommanderName.Length < 1 && this.type != 110 | this.Game.HandyFunctionsObj.GetUnitByHistorical(index) == -1)
            {
              Number += 1;
              Name: String;
              if (this.Data.HistoricalUnitObj[index].TempRegime > -1)
                Name = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[index].Name + " (" + this.Data.RegimeObj[this.Data.HistoricalUnitObj[index].TempRegime].Name + ")";
              else
                Name = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[index].Name;
              this.ListBox1.Items.Add( new ListItem(index, Name));
            }
          }
        }
        else if (this.type == 144)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No His Unit ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut ID: i32 =  0; ID <= historicalUnitCounter; ID += 1)
          {
            if (!this.Data.HistoricalUnitObj[ID].Model)
            {
              Number += 1;
              Name: String;
              if (this.Data.HistoricalUnitObj[ID].TempRegime > -1)
                Name = this.Data.HistoricalUnitObj[ID].Name + " (" + this.Data.RegimeObj[this.Data.HistoricalUnitObj[ID].TempRegime].Name + ") [" + Strings.Trim(Conversion.Str( Number)) + "]";
              else
                Name = this.Data.HistoricalUnitObj[ID].Name + " [" + Strings.Trim(Conversion.Str( Number)) + "]";
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 111 | this.type == 119 | this.type == 135)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Model ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut ID: i32 =  0; ID <= historicalUnitCounter; ID += 1)
          {
            if (this.Data.HistoricalUnitObj[ID].Model & this.Data.HistoricalUnitObj[ID].CommanderName.Length < 1)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[ID].Name;
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 112 | this.type == 121)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Officer ***"));
          let mut historicalUnitCounter: i32 =  this.Data.HistoricalUnitCounter;
          for (let mut ID: i32 =  0; ID <= historicalUnitCounter; ID += 1)
          {
            if (!this.Data.HistoricalUnitObj[ID].Model & this.Data.HistoricalUnitObj[ID].CommanderName.Length > 0)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.HistoricalUnitObj[ID].CommanderName;
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 113 | this.type == 120)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Landscape ***"));
          let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
          for (let mut ID: i32 =  0; ID <= landscapeTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LandscapeTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 114 | this.type == 122 | this.type == 132 | this.type == 134 | this.type == 138)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No People ***"));
          let mut peopleCounter: i32 =  this.Data.PeopleCounter;
          for (let mut ID: i32 =  0; ID <= peopleCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.PeopleObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 115 | this.type == 124)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No River ***"));
          let mut riverTypeCounter: i32 =  this.Data.RiverTypeCounter;
          for (let mut ID: i32 =  0; ID <= riverTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.RiverTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 116 | this.type == 125)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Road ***"));
          let mut roadTypeCounter: i32 =  this.Data.RoadTypeCounter;
          for (let mut ID: i32 =  0; ID <= roadTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.RoadTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 131 | this.type == 145)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No SFtype***"));
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut ID: i32 =  0; ID <= sfTypeCounter; ID += 1)
          {
            if (!this.Data.SFTypeObj[ID].DontShowInList)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.SFTypeObj[ID].Name;
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 117 | this.type == 126)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No SFtype***"));
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut ID: i32 =  0; ID <= sfTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.SFTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type ==  sbyte.MaxValue | this.type == 128)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No Loctype***"));
          let mut locTypeCounter: i32 =  this.Data.LocTypeCounter;
          for (let mut ID: i32 =  0; ID <= locTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LocTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 130)
        {
          let mut Number: i32 =  -1;
          let mut sfTypeCounter: i32 =  this.Data.SFTypeCounter;
          for (let mut ID: i32 =  0; ID <= sfTypeCounter; ID += 1)
          {
            if (this.Game.Data.SFTypeObj[ID].DontShowInList & Operators.CompareString(this.Game.Data.SFTypeObj[ID].Name, "Reserved SFType", false) != 0 && Strings.InStr(this.Game.Data.SFTypeObj[ID].Name.ToLower(), "unused") <= 0 && Strings.InStr(this.Game.Data.SFTypeObj[ID].Name.ToLower(), "n/a") <= 0)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.SFTypeObj[ID].Name;
              this.ListBox1.Items.Add( new ListItem(ID, Name));
              if (Number == 0)
                this.ListBox1.SelectedItem = RuntimeHelpers.GetObjectValue(this.ListBox1.Items[0]);
            }
          }
        }
        else if (this.type == 133)
        {
          this.ListBox1.Items.Add( new ListItem(1, "Individual Unit"));
          this.ListBox1.Items.Add( new ListItem(2, "Multi Unit"));
          this.ListBox1.Items.Add( new ListItem(5, "Lowest HQ"));
          this.ListBox1.Items.Add( new ListItem(6, "Medium HQ"));
          this.ListBox1.Items.Add( new ListItem(7, "High HQ"));
          this.ListBox1.Items.Add( new ListItem(8, "Supreme HQ"));
        }
        else if (this.type == 140 | this.type == 142)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***No Small Graphic***"));
          let mut smallPicCounter: i32 =  this.Data.SmallPicCounter;
          for (let mut ID: i32 =  0; ID <= smallPicCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.SmallPicName[ID];
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 141 | this.type == 143)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "***No Event Pic***"));
          let mut eventPicCounter: i32 =  this.Data.EventPicCounter;
          for (let mut ID: i32 =  0; ID <= eventPicCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.EventPicName[ID];
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 146)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-1, "*** No ActionCard ***"));
          let mut actionCardCounter: i32 =  this.Data.ActionCardCounter;
          for (let mut ID: i32 =  0; ID <= actionCardCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.ActionCardObj[ID].Title;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 147)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** Do NOT replace ***"));
          let mut landscapeTypeCounter: i32 =  this.Data.LandscapeTypeCounter;
          for (let mut ID: i32 =  0; ID <= landscapeTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LandscapeTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 148)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** Do NOT replace ***"));
          let mut roadTypeCounter: i32 =  this.Data.RoadTypeCounter;
          for (let mut ID: i32 =  0; ID <= roadTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.RoadTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 149)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** Do NOT replace ***"));
          let mut riverTypeCounter: i32 =  this.Data.RiverTypeCounter;
          for (let mut ID: i32 =  0; ID <= riverTypeCounter; ID += 1)
          {
            Number += 1;
            Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.RiverTypeObj[ID].Name;
            this.ListBox1.Items.Add( new ListItem(ID, Name));
          }
        }
        else if (this.type == 150)
        {
          let mut num43: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(0, "*** Nothing ***"));
          let mut stringListById: i32 =  this.Game.HandyFunctionsObj.GetStringListByID(this.Game.Data.StringListObj[this.nr].LookUpCol[this.nr3]);
          if (stringListById > -1)
          {
            let mut length: i32 =  this.Data.StringListObj[stringListById].Length;
            for (let mut index: i32 =  0; index <= length; index += 1)
            {
              num43 += 1;
              Name: String = this.Data.StringListObj[stringListById].Data[index, this.Data.StringListObj[stringListById].LookUpLabel] + " [" + this.Data.StringListObj[stringListById].Data[index, this.Data.StringListObj[stringListById].LookUpId] + "]";
              this.ListBox1.Items.Add( new ListItem( Math.Round(Conversion.Val(this.Data.StringListObj[stringListById].Data[index, this.Data.StringListObj[stringListById].LookUpId])), Name));
            }
          }
          ArrayList arrayList = ArrayList::new();
          let mut num44: i32 =  this.ListBox1.Items.Count - 1;
          for (let mut index: i32 =  0; index <= num44; index += 1)
            arrayList.Add(RuntimeHelpers.GetObjectValue(this.ListBox1.Items[index]));
          SortListArray sortListArray = new SortListArray(true);
          this.ListBox1.Items.Clear();
          let mut num45: i32 =  arrayList.Count - 1;
          for (let mut index: i32 =  0; index <= num45; index += 1)
            this.ListBox1.Items.Add(RuntimeHelpers.GetObjectValue(arrayList[index]));
        }
        else if (this.type == 151)
        {
          let mut Number: i32 =  -1;
          this.ListBox1.Items.Add( new ListItem(-2, "*** Do NOT replace ***"));
          let mut locTypeCounter: i32 =  this.Data.LocTypeCounter;
          for (let mut ID: i32 =  0; ID <= locTypeCounter; ID += 1)
          {
            if (!this.Data.LocTypeObj[ID].editorBlock)
            {
              Number += 1;
              Name: String = Strings.Trim(Conversion.Str( Number)) + ") " + this.Data.LocTypeObj[ID].Name;
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
        else if (this.type == 153)
        {
          num1 = -1;
          let mut num46: i32 =  1;
          do
          {
            Coordinate coordinate = this.Game.HandyFunctionsObj.HexNeighbourSameMap(this.Game.SelectX, this.Game.SelectY, 0, num46);
            if (coordinate.onmap && this.Game.Data.MapObj[0].HexObj[this.Game.SelectX, this.Game.SelectY].Bridge[num46 - 1] & this.Game.Data.MapObj[0].HexObj[this.Game.SelectX, this.Game.SelectY].RiverType[num46 - 1] > -1)
            {
              Name: String;
              if (num46 == 1)
                Name = "North".to_owned();
              if (num46 == 2)
                Name = "North-East";
              if (num46 == 3)
                Name = "South-East";
              if (num46 == 4)
                Name = "South".to_owned();
              if (num46 == 5)
                Name = "South-West";
              if (num46 == 6)
                Name = "North-West";
              Name = Name + " (" + coordinate.x.ToString() + "," + coordinate.y.ToString() + ")";
              this.ListBox1.Items.Add( new ListItem(num46, Name));
            }
            num46 += 1;
          }
          while (num46 <= 6);
        }
        else if (this.type == 154)
        {
          num1 = -1;
          CoordList coordList = this.Game.HandyFunctionsObj.InfraHexHighlight_getCoordList(this.Game.SelectX, this.Game.SelectY, 0, this.Game.EditObj.UnitSelected);
          let mut counter: i32 =  coordList.counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
          {
            Coordinate coordinate = coordList.coord[index];
            if (coordinate.onmap)
            {
              let mut ID: i32 =  this.Game.HandyFunctionsObj.HexFacing(this.Game.SelectX, this.Game.SelectY, 0, coordinate.x, coordinate.y, 0);
              Name: String;
              if (ID == 1)
                Name = "North".to_owned();
              if (ID == 2)
                Name = "North-East";
              if (ID == 3)
                Name = "South-East";
              if (ID == 4)
                Name = "South".to_owned();
              if (ID == 5)
                Name = "South-West";
              if (ID == 6)
                Name = "North-West";
              Name = Name + " (" + coordinate.x.ToString() + "," + coordinate.y.ToString() + ")";
              this.ListBox1.Items.Add( new ListItem(ID, Name));
            }
          }
        }
      }
      this.Button1.Text = "OK".to_owned();
      if (this.type == 1)
        this.Label1.Text = "Select a groundtype for landscapetype " + this.Data.LandscapeTypeObj[this.nr].Name;
      if (this.type == 2)
        this.Label1.Text = "Select a new landscape to be overriden by " + this.Data.LandscapeTypeObj[this.nr].Name;
      if (this.type == 3)
        this.Label1.Text = "Select a new people for " + this.Data.RegimeObj[this.nr].Name;
      if (this.type == 4)
        this.Label1.Text = "Select a movetype for " + this.Data.SFTypeObj[this.nr].Name;
      if (this.type == 5)
        this.Label1.Text = "Select a unitgroup for " + this.Data.SFTypeObj[this.nr].Name;
      if (this.type == 6)
        this.Label1.Text = "Select a SFType for subformation ";
      if (this.type == 7)
        this.Label1.Text = "Select a people for subformation ";
      if (this.type == 8)
        this.Label1.Text = "Select a Landscape Type for " + this.Data.LocTypeObj[this.nr].Name;
      if (this.type == 9)
        this.Label1.Text = "Select a Sprite for " + this.Data.LocTypeObj[this.nr].Name;
      if (this.type == 10)
        this.Label1.Text = "Select a Landscape Type for destructing " + this.Data.LocTypeObj[this.nr].Name;
      if (this.type == 11)
        this.Label1.Text = "Select a Sprite for destructing " + this.Data.LocTypeObj[this.nr].Name;
      if (this.type == 12)
        this.Label1.Text = "Select loctype group for " + this.Data.LocTypeObj[this.nr].Name;
      if (this.type == 13)
        this.Label1.Text = "Select peoplegroup for " + this.Data.PeopleObj[this.nr].Name;
      if (this.type == 14)
        this.Label1.Text = "Select regime for regimecol of " + this.Data.PeopleObj[this.nr].Name;
      if (this.type == 15)
        this.Label1.Text = "Select regime";
      if (this.type == 16)
        this.Label1.Text = "Select people for location " + this.Data.LocObj[this.nr].Name;
      if (this.type == 17)
        this.Label1.Text = "Select itemtypegroup for itemtype " + this.Data.ItemTypeObj[this.nr].Name;
      if (this.type == 18)
        this.Label1.Text = "Select researchfield needed for itemtype " + this.Data.ItemTypeObj[this.nr].Name;
      if (this.type == 19)
        this.Label1.Text = "Select sftype for itemtype " + this.Data.ItemTypeObj[this.nr].Name;
      if (this.type == 20)
        this.Label1.Text = "Select itemtype blocked by itemtype " + this.Data.ItemTypeObj[this.nr].Name;
      if (this.type == 21)
        this.Label1.Text = "Select sftype picture for researchfield " + this.Data.ResearchObj[this.nr].Name;
      if (this.type == 22 | this.type == 23)
        this.Label1.Text = "Select prereq research for " + this.Data.ResearchObj[this.nr].Name;
      if (this.type == 24)
        this.Label1.Text = "Select which field is blocked for " + this.Data.ResearchObj[this.nr].Name;
      if (this.type == 25)
        this.Label1.Text = "Select gameslot for variant # " + Conversion.Str( this.nr);
      if (this.type == 26)
        this.Label1.Text = "Select upgrade option for " + this.Data.SFTypeObj[this.nr].Name;
      if (this.type == 27)
        this.Label1.Text = "Select historical unit for unit# " + Conversion.Str( this.nr);
      if (this.type == 31)
        this.Label1.Text = "Select eventnr to be fired by playing actioncard: " + this.Data.ActionCardObj[this.nr].Title;
      if (this.type == 32)
        this.Label1.Text = "Select eventcategory: ";
      if (this.type == 33)
        this.Label1.Text = "Select new HQ for " + this.Data.LocObj[this.nr].Name + ":";
      if (this.type == 34)
        this.Label1.Text = "Select SFType: ";
      if (this.type == 35)
        this.Label1.Text = "Select eventnr for pre execute event nr: " + this.Data.ActionCardObj[this.nr].Title;
      if (this.type == 36)
        this.Label1.Text = "Select Type of Historical Unit: ";
      if (this.type == 37)
        this.Label1.Text = "Select Historical Unit: ";
      if (this.type == 38)
        this.Label1.Text = "Set CampaignRoom Auto ActionCard: ";
      if (this.type == 39)
        this.Label1.Text = "Set auto-event for historical unit";
      if (this.type == 40)
        this.Label1.Text = "Set action card for historical unit";
      if (this.type == 41)
        this.Label1.Text = "Set marker for unit";
      if (this.type == 42)
        this.Label1.Text = "Select SFType to View";
      if (this.type == 43)
        this.Label1.Text = "Select SFType to Copy";
      if (this.type == 44)
        this.Label1.Text = "Select Override MoveType for Subformation";
      if (this.type == 45)
        this.Label1.Text = "Select Predef unit";
      if (this.type == 46)
        this.Label1.Text = "Select historical unit to use as a MODEL";
      if (this.type == 47)
        this.Label1.Text = "Select historical unit to use as a MODEL";
      if (this.type == 48)
        this.Label1.Text = "Choose subpart for unit";
      if (this.type == 49)
        this.Label1.Text = "Choose peoplemod for itemtype";
      if (this.type == 50)
        this.Label1.Text = "Choose movetype for itemtype";
      if (this.type == 51)
        this.Label1.Text = "Choose reinforcementtype for sftype";
      if (this.type == 52)
        this.Label1.Text = "Choose ally to give unit too";
      if (this.type == 53)
        this.Label1.Text = "Choose ally to give selected hex too";
      if (this.type == 54)
        this.Label1.Text = "Choose ally to give selected research too";
      if (this.type == 55)
        this.Label1.Text = "Choose a stringlist for the officerpool algorithm";
      if (this.type == 56)
        this.Label1.Text = "Choose a format for this stringlist (or no format)";
      if (this.type == 57)
        this.Label1.Text = "Choose a UberRegime";
      if (this.type == 58)
        this.Label1.Text = "Select an event to immediatly execute upon change of this variant.";
      if (this.type == 59)
        this.Label1.Text = "Select an air unit to join in attack";
      if (this.type == 60)
        this.Label1.Text = "Select a HQ to provide CAP pts for strategic transfer";
      if (this.type == 61)
        this.Label1.Text = "Select a Landscapetype who's special 6/64 sprites to use for exterior transitions";
      if (this.type == 62)
        this.Label1.Text = "Select a new landscape to be overriden by " + this.Data.LandscapeTypeObj[this.nr].Name;
      if (this.type == 63)
        this.Label1.Text = "Pick Improvement Event for researchfield " + this.Data.ResearchObj[this.nr2].Name;
      if (this.type == 64)
        this.Label1.Text = "Pick event for new/upgrade ";
      if (this.type == 65)
        this.Label1.Text = "Pick stringlist for model names ";
      if (this.type == 66)
        this.Label1.Text = "Pick event for InitialEvent ";
      if (this.type == 67)
        this.Label1.Text = "Pick itemtype for model";
      if (this.type == 68)
        this.Label1.Text = "Pick SfTypeGroup for PreventHitOn";
      if (this.type == 69)
        this.Label1.Text = "Pick SfTypeGroup for PreventHitFrom";
      if (this.type == 70)
        this.Label1.Text = "Pick SfType to copy this attack table stats from";
      if (this.type == 71)
        this.Label1.Text = "Choose extra research necc to make new model or upgrade";
      if (this.type == 72)
        this.Label1.Text = "Choose research to make building loctype possible";
      if (this.type == 73)
        this.Label1.Text = "Choose regimeslot (resource) that is necc for constructing loctype";
      if (this.type == 74)
        this.Label1.Text = "Choose a trooptype to compare with";
      if (this.type == 75 | this.type == 76)
        this.Label1.Text = "Choose an event to call";
      if (this.type == 77 | this.type == 78)
        this.Label1.Text = "Choose an event to call";
      if (this.type == 79)
        this.Label1.Text = "Choose an alteration to make";
      if (this.type == 80)
        this.Label1.Text = "Choose an itemtype for autoprod of LT";
      if (this.type == 81)
        this.Label1.Text = "Choose an event to call";
      if (this.type == 82 & this.nr > -1)
        this.Label1.Text = "Choose an HQ to assign " + this.Data.UnitObj[this.nr].Name + " too.";
      if (this.type == 83)
        this.Label1.Text = "Choose a people for historical unit/commander.";
      if (this.type == 84)
        this.Label1.Text = "Choose historical unit model.";
      if (this.type == 85)
        this.Label1.Text = "Choose historical unit with deck cards";
      if (this.type == 86)
        this.Label1.Text = "Choose historical unit with hisvar variables";
      if (this.type == 87)
        this.Label1.Text = "Choose non-model historical unit not yet placed on map you want to put on map";
      if (this.type == 88)
        this.Label1.Text = "Choose a people for historical unit GfxUse";
      if (this.type == 89)
        this.Label1.Text = "Choose sec. reinforcementtype for sftype";
      if (this.type == 90)
        this.Label1.Text = "Select a library";
      if (this.type == 91)
        this.Label1.Text = "Select a type";
      if (this.type == 92)
        this.Label1.Text = "Select a valueType";
      if (this.type == 93)
        this.Label1.Text = "Select a library";
      if (this.type == 94)
        this.Label1.Text = "Select a library";
      if (this.type == 95 | this.type == 96 | this.type == 97)
        this.Label1.Text = "Select a library";
      if (this.type == 98)
        this.Label1.Text = "Substitute by which existing regime?";
      if (this.type == 99)
        this.Label1.Text = "Substitute by which existing people?";
      if (this.type == 100)
        this.Label1.Text = "Select MODEL to place an instance of on the map";
      if (this.type == 101)
        this.Label1.Text = "Select his.unit to place on the map";
      if (this.type == 102)
        this.Label1.Text = "Select commander to add to pool";
      if (this.type == 103)
        this.Label1.Text = "Select commander to assign to selected unit";
      if (this.type == 104)
        this.Label1.Text = "Select a library";
      if (this.type == 105)
        this.Label1.Text = "Select a valueType";
      if (this.type == 106)
        this.Label1.Text = "Select regime for stringlist cell";
      if (this.type == 107)
        this.Label1.Text = "Set Library of eventpic";
      if (this.type == 108)
        this.Label1.Text = "Set Library of small gfx";
      if (this.type == 109)
        this.Label1.Text = "Set Library of cards";
      if (this.type == 110)
        this.Label1.Text = "Set Historical Unit";
      if (this.type == 111)
        this.Label1.Text = "Set Model His Unit";
      if (this.type == 112)
        this.Label1.Text = "Set Commander Historical Unit";
      if (this.type == 113)
        this.Label1.Text = "Set Landscape";
      if (this.type == 114)
        this.Label1.Text = "Set People";
      if (this.type == 115)
        this.Label1.Text = "Set River";
      if (this.type == 116)
        this.Label1.Text = "Set Road";
      if (this.type == 117)
        this.Label1.Text = "Set Troop&Equip";
      if (this.type == 118)
        this.Label1.Text = "Set Historical";
      if (this.type == 119)
        this.Label1.Text = "Set His MODEL";
      if (this.type == 120)
        this.Label1.Text = "Set Landscape";
      if (this.type == 121)
        this.Label1.Text = "Set Commander";
      if (this.type == 122)
        this.Label1.Text = "Set People";
      if (this.type == 123)
        this.Label1.Text = "Set Regime";
      if (this.type == 124)
        this.Label1.Text = "Set River";
      if (this.type == 125)
        this.Label1.Text = "Set Road";
      if (this.type == 126)
        this.Label1.Text = "Set Troop&Equip";
      if (this.type ==  sbyte.MaxValue)
        this.Label1.Text = "Set LocType";
      if (this.type == 128)
        this.Label1.Text = "Set LocType";
      if (this.type == 129)
        this.Label1.Text = "Set Library of reinf.type";
      if (this.type == 130)
        this.Label1.Text = "Select based on which troop type?";
      if (this.type == 131)
        this.Label1.Text = "Select Troop&Equip";
      if (this.type == 132)
        this.Label1.Text = "Select People";
      if (this.type == 133)
        this.Label1.Text = "Select Type of historical unit";
      if (this.type == 134)
        this.Label1.Text = "Select People";
      if (this.type == 135)
        this.Label1.Text = "Select Model";
      if (this.type == 136)
        this.Label1.Text = "Select Unit";
      if (this.type == 137)
        this.Label1.Text = "Select People";
      if (this.type == 138)
        this.Label1.Text = "Select Model";
      if (this.type == 139)
        this.Label1.Text = "Select Deck card";
      if (this.type == 140)
        this.Label1.Text = "Select Small Gfx";
      if (this.type == 141)
        this.Label1.Text = "Select Event Picture";
      if (this.type == 142)
        this.Label1.Text = "Select Small Gfx";
      if (this.type == 143)
        this.Label1.Text = "Select Event Picture";
      if (this.type == 144)
        this.Label1.Text = "Set Historical Unit";
      if (this.type == 145)
        this.Label1.Text = "Set Troop&Equip";
      if (this.type == 146)
        this.Label1.Text = "Select Actioncard";
      if (this.type == 147)
        this.Label1.Text = "Select Replacement";
      if (this.type == 148)
        this.Label1.Text = "Select Replacement";
      if (this.type == 149)
        this.Label1.Text = "Select Replacement";
      if (this.type == 150)
        this.Label1.Text = "Select Lookup Id/Label";
      if (this.type == 151)
        this.Label1.Text = "Select Replacement";
      if (this.type == 153)
        this.Label1.Text = "Select bridge to blow";
      if (this.type == 154)
        this.Label1.Text = "Select bridge to construct/repair";
      this.Show();
      this.Focus();
    }

     void Button1_Click(object sender, EventArgs e)
    {
      if (this.ListBox1.SelectedIndex >= 0)
      {
        ListItem selectedItem = (ListItem) this.ListBox1.SelectedItem;
        num1: i32;
        if (this.type == 1)
          this.Data.LandscapeTypeObj[this.nr].BuildGround = selectedItem.ID;
        else if (this.type == 2)
          this.Data.LandscapeTypeObj[this.nr].AddOverride(selectedItem.ID);
        else if (this.type == 3)
          this.Data.RegimeObj[this.nr].People = selectedItem.ID;
        else if (this.type == 4)
          this.Data.SFTypeObj[this.nr].MoveType = selectedItem.ID;
        else if (this.type == 5)
          this.Data.SFTypeObj[this.nr].UnitGroup = selectedItem.ID;
        else if (this.type == 6)
          this.Data.SFObj[this.nr].Type = selectedItem.ID;
        else if (this.type == 7)
          this.Data.SFObj[this.nr].People = selectedItem.ID;
        else if (this.type == 8)
        {
          this.Data.LocTypeObj[this.nr].OverdrawLTNr = selectedItem.ID;
          this.Data.LocTypeObj[this.nr].OverdrawSpriteNr = 0;
        }
        else if (this.type == 9)
          this.Data.LocTypeObj[this.nr].OverdrawSpriteNr = selectedItem.ID;
        else if (this.type == 10)
        {
          this.Data.LocTypeObj[this.nr].OnDestructLT = selectedItem.ID;
          this.Data.LocTypeObj[this.nr].OnDestructSpriteNr = 0;
        }
        else if (this.type == 11)
          this.Data.LocTypeObj[this.nr].OnDestructSpriteNr = selectedItem.ID;
        else if (this.type == 12)
          this.Data.LocTypeObj[this.nr].LocTypeGroup = selectedItem.ID;
        else if (this.type == 13)
          this.Data.PeopleObj[this.nr].PeopleGroup = selectedItem.ID;
        else if (this.type == 14)
          this.Data.PeopleObj[this.nr].RegCol = selectedItem.ID;
        else if (this.type == 15)
          this.Data.HistoricalUnitObj[this.nr].TempRegime = selectedItem.ID;
        else if (this.type == 16)
          this.Data.LocObj[this.nr].People = selectedItem.ID;
        else if (this.type == 17)
          this.Data.ItemTypeObj[this.nr].ItemGroup = selectedItem.ID;
        else if (this.type == 18)
          this.Data.ItemTypeObj[this.nr].ResFieldNeeded[this.nr2] = selectedItem.ID;
        else if (this.type == 19)
          this.Data.ItemTypeObj[this.nr].IsSFType = selectedItem.ID;
        else if (this.type == 20)
          this.Data.ItemTypeObj[this.nr].Blocks = selectedItem.ID;
        else if (this.type == 21)
          this.Data.ResearchObj[this.nr].SFTypePic = selectedItem.ID;
        else if (this.type == 22)
          this.Data.ResearchObj[this.nr].PreReq = selectedItem.ID;
        else if (this.type == 23)
          this.Data.ResearchObj[this.nr].PreReq2 = selectedItem.ID;
        else if (this.type == 24)
          this.Data.ResearchObj[this.nr].Blocks = selectedItem.ID;
        else if (this.type == 25)
          this.Data.Variants[this.nr] = selectedItem.ID;
        else if (this.type == 26)
          this.Data.SFTypeObj[this.nr].UpgradeToo = selectedItem.ID;
        else if (this.type != 27 && this.type != 28 && this.type != 29 && this.type != 30)
        {
          if (this.type == 31)
            this.Data.ActionCardObj[this.nr].ExecuteEvent = selectedItem.ID;
          else if (this.type == 32)
            this.Data.EventObj[this.nr].Category = selectedItem.ID;
          else if (this.type == 33)
            this.Data.LocObj[this.nr].HQ = selectedItem.ID;
          else if (this.type == 34)
            this.Data.ItemTypeObj[this.nr].UseSFType = selectedItem.ID;
          else if (this.type == 35)
            this.Data.ActionCardObj[this.nr].PreExecuteEvent = selectedItem.ID;
          else if (this.type == 36)
            this.Data.HistoricalUnitObj[this.nr].Type = selectedItem.ID;
          else if (this.type == 37)
          {
            this.Data.UnitObj[this.nr].Historical = selectedItem.ID;
            if (selectedItem.ID > -1)
              this.Data.HistoricalUnitObj[selectedItem.ID].TempRegime = this.Data.UnitObj[this.nr].Regime;
          }
          else if (this.type == 38)
            this.Data.CampaignRoom = selectedItem.ID;
          else if (this.type == 39)
          {
            HistoricalUnitClass[] historicalUnitObj = this.Data.HistoricalUnitObj;
            HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
            let mut nr: i32 =  this.nr;
            let mut index: i32 =  nr;
            historicalUnitClassArray[index].AutoEventCounter = historicalUnitObj[nr].AutoEventCounter + 1;
            this.Data.HistoricalUnitObj[this.nr].AutoEvent = (int[]) Utils.CopyArray((Array) this.Data.HistoricalUnitObj[this.nr].AutoEvent, (Array) new int[this.Data.HistoricalUnitObj[this.nr].AutoEventCounter + 1]);
            this.Data.HistoricalUnitObj[this.nr].AutoChance = (int[]) Utils.CopyArray((Array) this.Data.HistoricalUnitObj[this.nr].AutoChance, (Array) new int[this.Data.HistoricalUnitObj[this.nr].AutoEventCounter + 1]);
            this.Data.HistoricalUnitObj[this.nr].AutoEvent[this.Data.HistoricalUnitObj[this.nr].AutoEventCounter] = selectedItem.ID;
            this.Data.HistoricalUnitObj[this.nr].AutoChance[this.Data.HistoricalUnitObj[this.nr].AutoEventCounter] = 10;
          }
          else if (this.type == 40)
          {
            HistoricalUnitClass[] historicalUnitObj = this.Data.HistoricalUnitObj;
            HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
            let mut nr: i32 =  this.nr;
            let mut index: i32 =  nr;
            historicalUnitClassArray[index].DeckCardCounter = historicalUnitObj[nr].DeckCardCounter + 1;
            this.Data.HistoricalUnitObj[this.nr].DeckCard = (int[]) Utils.CopyArray((Array) this.Data.HistoricalUnitObj[this.nr].DeckCard, (Array) new int[this.Data.HistoricalUnitObj[this.nr].DeckCardCounter + 1]);
            this.Data.HistoricalUnitObj[this.nr].DeckChance = (int[]) Utils.CopyArray((Array) this.Data.HistoricalUnitObj[this.nr].DeckChance, (Array) new int[this.Data.HistoricalUnitObj[this.nr].DeckCardCounter + 1]);
            this.Data.HistoricalUnitObj[this.nr].DeckCard[this.Data.HistoricalUnitObj[this.nr].DeckCardCounter] = selectedItem.ID;
            this.Data.HistoricalUnitObj[this.nr].DeckChance[this.Data.HistoricalUnitObj[this.nr].DeckCardCounter] = 10;
          }
          else if (this.type == 41)
          {
            let mut index1: i32 =  -1;
            let mut moveMatrixCounter: i32 =  this.Game.NewAIObj.MoveMatrixCounter;
            for (let mut index2: i32 =  0; index2 <= moveMatrixCounter; index2 += 1)
            {
              if (this.Game.NewAIObj.MoveMatrixUnit[index2] == this.Game.EditObj.OrderUnit)
              {
                index1 = index2;
                break;
              }
            }
            if (index1 > -1)
              this.Game.NewAIObj.MoveMatrixUnitMarker[index1] = selectedItem.ID;
          }
          else if (this.type == 42)
            DrawMod.TGame.EditObj.TempSelected = selectedItem.ID;
          else if (this.type == 43)
            DrawMod.TGame.EditObj.TempCopy = selectedItem.ID;
          else if (this.type == 44)
            DrawMod.TGame.Data.SFObj[this.nr].MoveType = selectedItem.ID;
          else if (this.type == 45)
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].SubParts[this.nr2] = selectedItem.ID;
          else if (this.type == 46)
          {
            if (selectedItem.ID > -1)
            {
              num1 = 0;
              let mut num2: i32 =  this.type != 47 ? 7 :  Interaction.MsgBox( "Do you want to overwrite te units composition with the MODELS?", MsgBoxStyle.YesNo);
              this.Data.HistoricalUnitObj[this.nr].Counter = this.Data.HistoricalUnitObj[selectedItem.ID].Counter;
              this.Data.HistoricalUnitObj[this.nr].Green = this.Data.HistoricalUnitObj[selectedItem.ID].Green;
              this.Data.HistoricalUnitObj[this.nr].SmallGfx = this.Data.HistoricalUnitObj[selectedItem.ID].SmallGfx;
              this.Data.HistoricalUnitObj[this.nr].Red = this.Data.HistoricalUnitObj[selectedItem.ID].Red;
              this.Data.HistoricalUnitObj[this.nr].People = this.Data.HistoricalUnitObj[selectedItem.ID].People;
              this.Data.HistoricalUnitObj[this.nr].Blue = this.Data.HistoricalUnitObj[selectedItem.ID].Blue;
              this.Data.HistoricalUnitObj[this.nr].Type = this.Data.HistoricalUnitObj[selectedItem.ID].Type;
              this.Data.HistoricalUnitObj[this.nr].TempRegime = this.Data.HistoricalUnitObj[selectedItem.ID].TempRegime;
              this.Data.HistoricalUnitObj[this.nr].ModelMaster = selectedItem.ID;
              let mut unitCounter1: i32 =  this.Data.UnitCounter;
              for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
              {
                if (this.Data.UnitObj[index].Historical == this.nr)
                  this.Data.UnitObj[index].HistoricalSubPart = -1;
              }
              let mut index3: i32 =  0;
              do
              {
                this.Data.HistoricalUnitObj[this.nr].SubParts[index3] = this.Data.HistoricalUnitObj[selectedItem.ID].SubParts[index3];
                this.Data.HistoricalUnitObj[this.nr].Designation[index3] = this.Data.HistoricalUnitObj[selectedItem.ID].Designation[index3];
                this.Data.HistoricalUnitObj[this.nr].DesignationSmall[index3] = this.Data.HistoricalUnitObj[selectedItem.ID].DesignationSmall[index3];
                if (this.Data.HistoricalUnitObj[this.nr].SubParts[index3] > -1)
                {
                  let mut unitCounter2: i32 =  this.Data.UnitCounter;
                  for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
                  {
                    if (this.Data.UnitObj[unr].Historical == this.nr & this.Data.UnitObj[unr].HistoricalSubPart == -1)
                    {
                      this.Data.UnitObj[unr].HistoricalSubPart = index3;
                      if (num2 == 6)
                      {
                        let mut hq: i32 =  this.Data.UnitObj[unr].HQ;
                        DrawMod.TGame.HandyFunctionsObj.CopyUnit(unr, DrawMod.TGame.HandyFunctionsObj.GetPreDef(this.Data.HistoricalUnitObj[this.nr].SubParts[index3]), false);
                        this.Data.UnitObj[unr].HQ = hq;
                        break;
                      }
                      break;
                    }
                  }
                }
                index3 += 1;
              }
              while (index3 <= 9);
            }
            else
            {
              this.Data.HistoricalUnitObj[this.nr].ModelMaster = selectedItem.ID;
              let mut unitCounter: i32 =  this.Data.UnitCounter;
              for (let mut index: i32 =  0; index <= unitCounter; index += 1)
              {
                if (this.Data.UnitObj[index].Historical == this.nr)
                  this.Data.UnitObj[index].HistoricalSubPart = -1;
              }
            }
          }
          else if (this.type == 47)
          {
            if (selectedItem.ID > -1 & this.nr > -1 & this.nr2 > -1)
            {
              num1 = 0;
              bool flag = false;
              if (Interaction.MsgBox( "Auto name and shortname?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                this.Game.ProcessingObj.AddNewUnitBasedOnHistorical(this.Game.SelectX, this.Game.SelectY, 0, this.Game.Data.MapObj[0].HexObj[this.Game.SelectX, this.Game.SelectY].Regime, selectedItem.ID, freePPnoUnit: true);
                flag = true;
                this.nr = this.Data.HistoricalUnitCounter;
              }
              if (!flag)
              {
                this.Data.AddHistoricalUnit();
                this.nr = this.Data.HistoricalUnitCounter;
                this.Data.HistoricalUnitObj[this.nr].Counter = this.Data.HistoricalUnitObj[selectedItem.ID].Counter;
                this.Data.HistoricalUnitObj[this.nr].Name = this.Data.HistoricalUnitObj[selectedItem.ID].Name;
                if (this.Data.HistoricalUnitObj[selectedItem.ID].NameCounter == -1 & this.Data.HistoricalUnitObj[selectedItem.ID].Type < 5)
                  this.Data.HistoricalUnitObj[this.nr].CounterString = this.Data.HistoricalUnitObj[selectedItem.ID].CounterString;
                else if (!flag)
                {
                  this.Data.HistoricalUnitObj[this.nr].Name = Interaction.InputBox("Give historical unit name (example: 22nd inf div, III Corps)", "Shadow Empire : Planetary Conquest");
                  this.Data.HistoricalUnitObj[this.nr].CounterString = Interaction.InputBox("Give CounterString(shortname) (22, III)", "Shadow Empire : Planetary Conquest");
                }
                this.Data.HistoricalUnitObj[this.nr].Green = this.Data.HistoricalUnitObj[selectedItem.ID].Green;
                this.Data.HistoricalUnitObj[this.nr].Red = this.Data.HistoricalUnitObj[selectedItem.ID].Red;
                this.Data.HistoricalUnitObj[this.nr].Blue = this.Data.HistoricalUnitObj[selectedItem.ID].Blue;
                this.Data.HistoricalUnitObj[this.nr].Type = this.Data.HistoricalUnitObj[selectedItem.ID].Type;
                this.Data.HistoricalUnitObj[this.nr].TempRegime = this.Data.HistoricalUnitObj[selectedItem.ID].TempRegime;
                if (this.Data.HistoricalUnitObj[this.nr].TempRegime == -1)
                  this.Data.HistoricalUnitObj[this.nr].TempRegime = this.Data.MapObj[0].HexObj[DrawMod.TGame.SelectX, DrawMod.TGame.SelectY].Regime;
                if (this.Data.HistoricalUnitObj[this.nr].TempRegime > -1)
                  this.Data.HistoricalUnitObj[this.nr].ModelMaster = selectedItem.ID;
              }
              let mut num3: i32 =  9999;
              let mut num4: i32 =  -1;
              let mut num5: i32 =  this.Game.Data.UnitObj[this.Game.Data.UnitCounter].Regime;
              if (num5 == -1)
                num5 = this.Game.Data.HistoricalUnitObj[this.nr].TempRegime;
              let mut unitCounter3: i32 =  this.Game.Data.UnitCounter;
              for (let mut index: i32 =  0; index <= unitCounter3; index += 1)
              {
                if (this.Game.Data.UnitObj[index].Regime == num5 && this.Game.Data.UnitObj[index].PreDef == -1 && this.Game.Data.UnitObj[index].IsHQ && index != this.Game.Data.UnitCounter)
                {
                  let mut num6: i32 =  this.Game.HandyFunctionsObj.Distance(DrawMod.TGame.Data.UnitObj[index].X, DrawMod.TGame.Data.UnitObj[index].Y, DrawMod.TGame.Data.UnitObj[index].Map, DrawMod.TGame.SelectX, DrawMod.TGame.SelectY, DrawMod.TGame.EditObj.MapSelected);
                  if (num6 < num3)
                  {
                    num3 = num6;
                    num4 = index;
                  }
                }
              }
              if (!flag)
              {
                let mut index: i32 =  0;
                do
                {
                  this.Data.HistoricalUnitObj[this.nr].SubParts[index] = this.Data.HistoricalUnitObj[selectedItem.ID].SubParts[index];
                  this.Data.HistoricalUnitObj[this.nr].Designation[index] = this.Data.HistoricalUnitObj[selectedItem.ID].Designation[index];
                  if (this.Data.HistoricalUnitObj[this.nr].SubParts[index] > -1)
                  {
                    this.Data.AddUnit(DrawMod.TGame.SelectX, DrawMod.TGame.SelectY, DrawMod.TGame.EditObj.MapSelected);
                    let mut unitCounter4: i32 =  this.Data.UnitCounter;
                    this.Data.UnitObj[unitCounter4].Name = this.Data.HistoricalUnitObj[this.nr].Name;
                    this.Data.UnitObj[unitCounter4].Historical = this.nr;
                    this.Data.UnitObj[unitCounter4].Regime = this.Data.HistoricalUnitObj[this.nr].TempRegime;
                    this.Data.UnitObj[unitCounter4].HistoricalSubPart = index;
                    DrawMod.TGame.HandyFunctionsObj.CopyUnit(unitCounter4, DrawMod.TGame.HandyFunctionsObj.GetPreDef(this.Data.HistoricalUnitObj[this.nr].SubParts[index]), false);
                    this.Data.UnitObj[unitCounter4].HQ = num4;
                  }
                  index += 1;
                }
                while (index <= 9);
              }
              else
              {
                let mut unitCounter5: i32 =  this.Game.Data.UnitCounter;
                for (let mut unr: i32 =  0; unr <= unitCounter5; unr += 1)
                {
                  if (this.Game.Data.UnitObj[unr].Historical == this.Game.Data.HistoricalUnitCounter)
                  {
                    let mut historicalSubPart: i32 =  this.Data.UnitObj[unr].HistoricalSubPart;
                    DrawMod.TGame.HandyFunctionsObj.CopyUnit(unr, DrawMod.TGame.HandyFunctionsObj.GetPreDef(this.Data.HistoricalUnitObj[this.nr].SubParts[historicalSubPart]), false);
                    this.Data.UnitObj[unr].HQ = num4;
                  }
                }
              }
            }
            else
            {
              this.Data.RemoveHistoricalUnit(this.Data.HistoricalUnitCounter);
              let mut num7: i32 =   Interaction.MsgBox( "Aborted because selected hex has no owner");
            }
          }
          else if (this.type == 48)
            DrawMod.TGame.Data.UnitObj[this.nr].HistoricalSubPart = selectedItem.ID;
          else if (this.type == 49)
            DrawMod.TGame.Data.ItemTypeObj[this.nr].PeopleMod = selectedItem.ID;
          else if (this.type == 50)
            DrawMod.TGame.Data.ItemTypeObj[this.nr].MoveTypeMod = selectedItem.ID;
          else if (this.type == 51)
            DrawMod.TGame.Data.SFTypeObj[this.nr].ReinforcementType = selectedItem.ID;
          else if (this.type == 52)
          {
            DrawMod += 1.TGame.Data.StepNr;
            if (!DrawMod.TGame.Data.UnitObj[this.nr].IsHQ)
            {
              let mut num8: i32 =  0;
              if (DrawMod.TGame.Data.UnitObj[this.nr].Historical > -1)
              {
                let mut unitCounter: i32 =  DrawMod.TGame.Data.UnitCounter;
                for (let mut index: i32 =  0; index <= unitCounter; index += 1)
                {
                  if (DrawMod.TGame.Data.UnitObj[index].Historical == DrawMod.TGame.Data.UnitObj[this.nr].Historical && DrawMod.TGame.Data.UnitObj[index].HQ == DrawMod.TGame.Data.UnitObj[this.nr].HQ && index != this.nr)
                  {
                    if (DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[index].Regime].UberRegime != DrawMod.TGame.Data.Turn && DrawMod.TGame.Data.RegimeObj[selectedItem.ID].UberRegime != DrawMod.TGame.Data.Turn)
                      DrawMod.TGame.Data.UnitObj[index].HQ = -1;
                    DrawMod.TGame.Data.UnitObj[index].Regime = selectedItem.ID;
                    DrawMod.TGame.Data.UnitObj[index].UnitIsGiven = true;
                    num8 += 1;
                  }
                }
              }
              if (DrawMod.TGame.Data.UnitObj[this.nr].Regime != DrawMod.TGame.Data.Turn)
                DrawMod.TGame.HandyFunctionsObj.AddMessageForOne(DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + " has taken over command of " + DrawMod.TGame.Data.UnitObj[this.nr].Name, DrawMod.TGame.Data.UnitObj[this.nr].Regime, -1, 1);
              if (DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[this.nr].Regime].UberRegime != DrawMod.TGame.Data.Turn && DrawMod.TGame.Data.RegimeObj[selectedItem.ID].UberRegime != DrawMod.TGame.Data.Turn)
                DrawMod.TGame.Data.UnitObj[this.nr].HQ = -1;
              DrawMod.TGame.Data.UnitObj[this.nr].Regime = selectedItem.ID;
              DrawMod.TGame.Data.UnitObj[this.nr].UnitIsGiven = true;
              DrawMod.TGame.HandyFunctionsObj.HistoryAddHex(DrawMod.TGame.Data.UnitObj[this.nr].X, DrawMod.TGame.Data.UnitObj[this.nr].Y, DrawMod.TGame.Data.UnitObj[this.nr].Map, DrawMod.TGame.Data.Turn, infostring: "Giving unit");
              DrawMod.TGame.HandyFunctionsObj.AddMessageForOne(DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + " has given command of " + DrawMod.TGame.Data.UnitObj[this.nr].Name + " to you.", selectedItem.ID, -1, 1);
            }
            else
            {
              let mut Number: i32 =  0;
              let mut unitCounter: i32 =  DrawMod.TGame.Data.UnitCounter;
              for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
              {
                if (DrawMod.TGame.HandyFunctionsObj.IsUnitInHQChain(unr, this.nr))
                {
                  Number += 1;
                  DrawMod.TGame.Data.UnitObj[unr].Regime = selectedItem.ID;
                  DrawMod.TGame.Data.UnitObj[unr].UnitIsGiven = true;
                  DrawMod.TGame.HandyFunctionsObj.HistoryAddHex(DrawMod.TGame.Data.UnitObj[unr].X, DrawMod.TGame.Data.UnitObj[unr].Y, DrawMod.TGame.Data.UnitObj[unr].Map, DrawMod.TGame.Data.Turn, infostring: "Giving unit");
                }
              }
              if (DrawMod.TGame.Data.UnitObj[this.nr].Regime != DrawMod.TGame.Data.Turn)
                DrawMod.TGame.HandyFunctionsObj.AddMessageForOne(DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + " has taken over command of " + DrawMod.TGame.Data.UnitObj[this.nr].Name + " and " + Conversion.Str( Number) + " subordinate units..", DrawMod.TGame.Data.UnitObj[this.nr].Regime, -1, 1);
              if (DrawMod.TGame.Data.RegimeObj[selectedItem.ID].UberRegime != DrawMod.TGame.Data.Turn && DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[this.nr].Regime].UberRegime != DrawMod.TGame.Data.Turn)
                DrawMod.TGame.Data.UnitObj[this.nr].HQ = -1;
              DrawMod.TGame.Data.UnitObj[this.nr].Regime = selectedItem.ID;
              DrawMod.TGame.Data.UnitObj[this.nr].UnitIsGiven = true;
              DrawMod.TGame.HandyFunctionsObj.HistoryAddHex(DrawMod.TGame.Data.UnitObj[this.nr].X, DrawMod.TGame.Data.UnitObj[this.nr].Y, DrawMod.TGame.Data.UnitObj[this.nr].Map, DrawMod.TGame.Data.Turn, infostring: "Giving unit");
              DrawMod.TGame.HandyFunctionsObj.AddMessageForOne(DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + " has given command of " + DrawMod.TGame.Data.UnitObj[this.nr].Name + " and " + Conversion.Str( Number) + " subordinate units to you.", selectedItem.ID, -1, 1);
            }
          }
          else if (this.type == 53)
          {
            DrawMod += 1.TGame.Data.StepNr;
            let mut Number1: i32 =  0;
            let mut Number2: i32 =  0;
            let mut num9: i32 =  DrawMod.TGame.SelectX - (this.nr + 2);
            let mut num10: i32 =  DrawMod.TGame.SelectX + (this.nr + 2);
            for (let mut index4: i32 =  num9; index4 <= num10; index4 += 1)
            {
              let mut num11: i32 =  DrawMod.TGame.SelectY - (this.nr + 2);
              let mut num12: i32 =  DrawMod.TGame.SelectY + (this.nr + 2);
              for (let mut index5: i32 =  num11; index5 <= num12; index5 += 1)
              {
                if (index4 >= 0 & index5 >= 0 & index4 <= DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].MapWidth & index5 <= DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].MapHeight && DrawMod.TGame.HandyFunctionsObj.Distance(DrawMod.TGame.SelectX, DrawMod.TGame.SelectY, DrawMod.TGame.EditObj.MapSelected, index4, index5, DrawMod.TGame.EditObj.MapSelected) <= this.nr && DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].HexObj[index4, index5].Regime == DrawMod.TGame.Data.Turn)
                {
                  DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].HexObj[index4, index5].Regime = selectedItem.ID;
                  DrawMod.TGame.HandyFunctionsObj.HistoryAddHex(index4, index5, DrawMod.TGame.EditObj.MapSelected, DrawMod.TGame.Data.Turn, infostring: "Giving hex");
                  Number2 += 1;
                  if (DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].HexObj[index4, index5].Location > -1)
                  {
                    DrawMod.TGame.Data.LocObj[DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].HexObj[index4, index5].Location].HQ = -1;
                    Number1 += 1;
                  }
                }
              }
            }
            DrawMod.TGame.HandyFunctionsObj.AddMessageForOne(DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + " has given " + Conversion.Str( Number2) + " hexes and " + Conversion.Str( Number1) + " locations to you.", selectedItem.ID, -1, 1);
          }
          else if (this.type == 54)
          {
            DrawMod.TGame.Data.RegimeObj[selectedItem.ID].ResField[this.nr] = true;
            DrawMod.TGame.HandyFunctionsObj.AddMessageForOne(DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.Turn].Name + " has given " + DrawMod.TGame.Data.ResearchObj[this.nr].Name + " research to you.", selectedItem.ID, -1, 1);
            let mut sfTypeCounter: i32 =  DrawMod.TGame.Data.SFTypeCounter;
            for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
            {
              if (DrawMod.TGame.Data.SFTypeObj[index].ModelID > 0 & DrawMod.TGame.Data.SFTypeObj[index].ModelRegime == selectedItem.ID)
              {
                let mut modelItemType: i32 =  DrawMod.TGame.Data.SFTypeObj[index].ModelItemType;
                let mut tv9: i32 =  index;
                let mut tv7: i32 =  0;
                let mut tv8: i32 =  0;
                if (DrawMod.TGame.Data.SFTypeObj[index].ModelAutoImprovement[this.nr] && DrawMod.TGame.Data.SFTypeObj[index].ModelLastState[this.nr] == 0 & DrawMod.TGame.Data.SFTypeObj[index].ModelImproveEvent[this.nr] > 0 && DrawMod.TGame.Data.RegimeObj[selectedItem.ID].ResField[this.nr])
                {
                  DrawMod.TGame.EventRelatedObj.DoCheckSpecificEvent(DrawMod.TGame.Data.SFTypeObj[index].ModelImproveEvent[this.nr], tv9: tv9, tv7: tv7, tv8: tv8, tv10: modelItemType);
                  DrawMod.TGame.Data.SFTypeObj[index].ModelLastState[this.nr] = 1;
                }
              }
            }
            SimpleList simpleList = SimpleList::new();
            let mut itemTypeCounter: i32 =  DrawMod.TGame.Data.ItemTypeCounter;
            for (let mut itemtypenr: i32 =  0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
            {
              if (DrawMod.TGame.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[0] == this.nr | DrawMod.TGame.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[1] == this.nr | DrawMod.TGame.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[2] == this.nr | DrawMod.TGame.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[3] == this.nr | DrawMod.TGame.Data.ItemTypeObj[itemtypenr].ResFieldNeeded[4] == this.nr && DrawMod.TGame.Data.ItemTypeObj[itemtypenr].Blocks > -1)
              {
                let mut blocks: i32 =  DrawMod.TGame.Data.ItemTypeObj[itemtypenr].Blocks;
                let mut locCounter: i32 =  DrawMod.TGame.Data.LocCounter;
                for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
                {
                  if (DrawMod.TGame.Data.MapObj[DrawMod.TGame.EditObj.MapSelected].HexObj[DrawMod.TGame.Data.LocObj[locnr].X, DrawMod.TGame.Data.LocObj[locnr].Y].Regime == selectedItem.ID)
                  {
                    let mut index: i32 =  0;
                    do
                    {
                      if (DrawMod.TGame.Data.LocObj[locnr].Production[index] == DrawMod.TGame.Data.ItemTypeObj[itemtypenr].Blocks && DrawMod.TGame.HandyFunctionsObj.CanProduceItem(locnr, selectedItem.ID, itemtypenr).result)
                      {
                        DrawMod.TGame.Data.LocObj[locnr].Production[index] = itemtypenr;
                        num13: i32;
                        num13 += 1;
                      }
                      index += 1;
                    }
                    while (index <= 3);
                  }
                }
              }
            }
          }
          else if (this.type == 55)
            DrawMod.TGame.Data.RegimeObj[this.nr].OfficerPool = selectedItem.ID;
          else if (this.type == 56)
          {
            if (selectedItem.ID == 0)
            {
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(0, "ID");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[0] = NewEnums.LibVarValueType.Number;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(1, "Type");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[1] = NewEnums.LibVarValueType.Number;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(2, "Parent");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[2] = NewEnums.LibVarValueType.Number;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(3, "Z-order");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[3] = NewEnums.LibVarValueType.Number;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(4, "Title");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[4] = NewEnums.LibVarValueType.Text;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(5, "Style");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[5] = NewEnums.LibVarValueType.Number;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(6, "Event");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[6] = NewEnums.LibVarValueType.Number;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(7, "Indicator");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[7] = NewEnums.LibVarValueType.SmallGfxId;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(8, "Text");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[8] = NewEnums.LibVarValueType.Text;
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(9, "AutoGray");
              DrawMod.TGame.Data.StringListObj[this.nr].ColValueType[9] = NewEnums.LibVarValueType.YesNo;
              DrawMod.TGame.Data.StringListObj[this.nr].Name = "UDS Management Tabs Format";
            }
            if (DrawMod.TGame.Data.StringListObj[this.nr].Width == -1)
              DrawMod.TGame.Data.StringListObj[this.nr].AddCol(-1);
            if (DrawMod.TGame.Data.StringListObj[this.nr].Length == -1)
              DrawMod.TGame.Data.StringListObj[this.nr].AddRow(-1);
          }
          else if (this.type == 57)
            DrawMod.TGame.Data.RegimeObj[this.nr].UberRegime = selectedItem.ID;
          else if (this.type == 58)
            DrawMod.TGame.Data.VariantEvent[this.nr] = selectedItem.ID;
          else if (this.type == 59)
          {
            if (DrawMod.TGame.EditObj.TempUnitList.CheckIfPresent(selectedItem.ID))
              DrawMod.TGame.EditObj.TempUnitList.remove(selectedItem.ID);
            else
              DrawMod.TGame.EditObj.TempUnitList.add(selectedItem.ID);
          }
          else if (this.type == 60)
            DrawMod.TGame.EditObj.OrderTarget = selectedItem.ID;
          else if (this.type == 61)
            DrawMod.TGame.Data.LandscapeTypeObj[this.nr].ExtraExterior = selectedItem.ID;
          else if (this.type == 62)
            this.Data.LandscapeTypeObj[this.nr].AddOverride2(selectedItem.ID);
          else if (this.type == 63)
            this.Data.SFTypeObj[this.nr].ModelImproveEvent[this.nr2] = selectedItem.ID;
          else if (this.type == 64)
            this.Data.SFTypeObj[this.nr].ModelNewEvent = selectedItem.ID;
          else if (this.type == 65)
            this.Data.SFTypeObj[this.nr].ModelNameList = selectedItem.ID;
          else if (this.type == 66)
            this.Data.SFTypeObj[this.nr].ModelInitialEvent = selectedItem.ID;
          else if (this.type == 67)
            this.Data.SFTypeObj[this.nr].ModelItemType = selectedItem.ID;
          else if (this.type == 68)
            this.Data.SFTypeObj[this.nr].PreventHitOn[this.nr2] = selectedItem.ID;
          else if (this.type == 69)
            this.Data.SFTypeObj[this.nr].PreventHitFrom[this.nr2] = selectedItem.ID;
          else if (this.type == 70)
          {
            let mut index: i32 =  0;
            do
            {
              this.Data.SFTypeObj[this.nr].FavTarget[index] = this.Data.SFTypeObj[selectedItem.ID].FavTarget[index];
              this.Data.SFTypeObj[this.nr].AttackPower[index] = this.Data.SFTypeObj[selectedItem.ID].AttackPower[index];
              this.Data.SFTypeObj[this.nr].AttackPowerDef[index] = this.Data.SFTypeObj[selectedItem.ID].AttackPowerDef[index];
              this.Data.SFTypeObj[this.nr].AttackArt[index] = RuntimeHelpers.GetObjectValue(this.Data.SFTypeObj[selectedItem.ID].AttackArt[index]);
              this.Data.SFTypeObj[this.nr].FavArtTarget[index] = this.Data.SFTypeObj[selectedItem.ID].FavArtTarget[index];
              this.Data.SFTypeObj[this.nr].HitPoints[index] = this.Data.SFTypeObj[selectedItem.ID].HitPoints[index];
              this.Data.SFTypeObj[this.nr].HitPointsDef[index] = this.Data.SFTypeObj[selectedItem.ID].HitPointsDef[index];
              index += 1;
            }
            while (index <= 99);
          }
          else if (this.type == 71)
            this.Data.SFTypeObj[this.nr].ModelExtraResearch = selectedItem.ID;
          else if (this.type == 72)
            this.Data.LocTypeObj[this.nr].Research[this.nr2] = selectedItem.ID;
          else if (this.type == 73)
            this.Data.LocTypeObj[this.nr].VarType[this.nr2] = selectedItem.ID;
          else if (this.type == 74)
            DrawMod.TGame.EditObj.SFCompare = selectedItem.ID;
          else if (this.type == 75)
            DrawMod.TGame.Data.LocTypeObj[this.nr].AIEvent = selectedItem.ID;
          else if (this.type == 76)
            DrawMod.TGame.Data.LocTypeObj[this.nr].AILocEvent = selectedItem.ID;
          else if (this.type == 77)
            DrawMod.TGame.Data.SFTypeObj[this.nr].ModelVariantCheck[this.nr2] = selectedItem.ID;
          else if (this.type == 78)
            DrawMod.TGame.Data.SFTypeObj[this.nr].ModelVariantExec[this.nr2] = selectedItem.ID;
          else if (this.type == 79)
          {
            if (selectedItem.ID > -1 && DrawMod.TGame.Data.SFTypeObj[this.nr].ModelVariantExec[selectedItem.ID] > -1)
            {
              DrawMod.TGame.EventRelatedObj.DoCheckSpecificEvent(DrawMod.TGame.Data.SFTypeObj[this.nr].ModelVariantExec[selectedItem.ID], tv9: this.nr);
              let mut num14: i32 =   Interaction.MsgBox( "Alteration to model has been made", Title: ( "Shadow Empire : Planetary Conquest"));
            }
          }
          else if (this.type == 80)
            DrawMod.TGame.Data.LocTypeObj[this.nr].AutoProd = selectedItem.ID;
          else if (this.type == 81)
            DrawMod.TGame.Data.LocTypeObj[this.nr].AIAfterBuildEvent = selectedItem.ID;
          else if (this.type == 82)
            DrawMod.TGame.ProcessingObj.SetUnitHq(this.nr, selectedItem.ID);
          else if (this.type == 83)
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].People = selectedItem.ID;
          else if (this.type == 84)
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].UseModelCounter = selectedItem.ID;
          else if (this.type == 85)
          {
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckCardCounter = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].DeckCardCounter;
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckCard = new int[DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckCardCounter + 1];
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckChance = new int[DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckCardCounter + 1];
            let mut deckCardCounter: i32 =  DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].DeckCardCounter;
            for (let mut index: i32 =  0; index <= deckCardCounter; index += 1)
            {
              DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckCard[index] = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].DeckCard[index];
              DrawMod.TGame.Data.HistoricalUnitObj[this.nr].DeckChance[index] = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].DeckChance[index];
            }
          }
          else if (this.type == 86)
          {
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarCount = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].HisVarCount;
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarNato = new int[DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarCount + 1];
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarType = new int[DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarCount + 1];
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarValue = new int[DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarCount + 1];
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarSmall = new int[DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarCount + 1];
            let mut hisVarCount: i32 =  DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].HisVarCount;
            for (let mut index: i32 =  0; index <= hisVarCount; index += 1)
            {
              DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarNato[index] = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].HisVarNato[index];
              DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarType[index] = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].HisVarType[index];
              DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarValue[index] = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].HisVarValue[index];
              DrawMod.TGame.Data.HistoricalUnitObj[this.nr].HisVarSmall[index] = DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].HisVarSmall[index];
            }
          }
          else if (this.type == 87)
          {
            this.nr = selectedItem.ID;
            if (this.Data.HistoricalUnitObj[this.nr].TempRegime > -1)
            {
              let mut num15: i32 =  9999;
              let mut num16: i32 =  -1;
              let mut unitCounter6: i32 =  this.Game.Data.UnitCounter;
              for (let mut index: i32 =  0; index <= unitCounter6; index += 1)
              {
                if (this.Game.Data.UnitObj[index].Regime == this.Data.HistoricalUnitObj[this.nr].TempRegime && this.Game.Data.UnitObj[index].Historical > -1 && this.Game.Data.HistoricalUnitObj[this.nr].Type < this.Game.Data.HistoricalUnitObj[this.Game.Data.UnitObj[index].Historical].Type && this.Game.Data.UnitObj[index].PreDef == -1 && this.Game.Data.UnitObj[index].IsHQ)
                {
                  let mut num17: i32 =  this.Game.HandyFunctionsObj.Distance(DrawMod.TGame.Data.UnitObj[index].X, DrawMod.TGame.Data.UnitObj[index].Y, DrawMod.TGame.Data.UnitObj[index].Map, DrawMod.TGame.SelectX, DrawMod.TGame.SelectY, DrawMod.TGame.EditObj.MapSelected);
                  if (num17 < num15)
                  {
                    num15 = num17;
                    num16 = index;
                  }
                }
              }
              let mut index6: i32 =  0;
              do
              {
                if (this.Data.HistoricalUnitObj[this.nr].SubParts[index6] > -1)
                {
                  this.Data.AddUnit(DrawMod.TGame.SelectX, DrawMod.TGame.SelectY, DrawMod.TGame.EditObj.MapSelected);
                  let mut unitCounter7: i32 =  this.Data.UnitCounter;
                  this.Data.UnitObj[unitCounter7].Name = this.Data.HistoricalUnitObj[this.nr].Name;
                  this.Data.UnitObj[unitCounter7].Historical = this.nr;
                  this.Data.UnitObj[unitCounter7].Regime = this.Data.HistoricalUnitObj[this.nr].TempRegime;
                  this.Data.UnitObj[unitCounter7].HistoricalSubPart = index6;
                  DrawMod.TGame.HandyFunctionsObj.CopyUnit(unitCounter7, DrawMod.TGame.HandyFunctionsObj.GetPreDef(this.Data.HistoricalUnitObj[this.nr].SubParts[index6]), false);
                  this.Data.UnitObj[unitCounter7].HQ = num16;
                }
                index6 += 1;
              }
              while (index6 <= 9);
            }
          }
          else if (this.type == 88)
            DrawMod.TGame.Data.HistoricalUnitObj[this.nr].UsePeopleGfx = selectedItem.ID;
          else if (this.type == 89)
            DrawMod.TGame.Data.SFTypeObj[this.nr].ReinforcementType2 = selectedItem.ID;
          else if (this.type == 90)
            this.Data.EventObj[this.nr].LibId.SetFromAdvancedEditor( this.Data, selectedItem.ID);
          else if (this.type == 91)
            this.Data.LibVarObj[this.nr].type = (NewEnums.LibVarType) selectedItem.ID;
          else if (this.type == 92)
            this.Data.LibVarObj[this.nr].valueType = (NewEnums.LibVarValueType) selectedItem.ID;
          else if (this.type == 93)
            this.Data.SFTypeObj[this.nr].LibId.SetFromAdvancedEditor( this.Data, selectedItem.ID);
          else if (this.type == 94)
            this.Data.HistoricalUnitObj[this.nr].LibId.SetFromAdvancedEditor( this.Data, selectedItem.ID);
          else if (this.type == 95)
            this.Data.UnitObj[this.nr].LibId.SetFromAdvancedEditor( this.Data, selectedItem.ID);
          else if (this.type == 96)
            this.Data.PeopleObj[this.nr].LibId.SetFromAdvancedEditor( this.Data, selectedItem.ID);
          else if (this.type == 97)
            this.Data.RegimeObj[this.nr].libId.SetFromAdvancedEditor( this.Data, selectedItem.ID);
          else if (this.type == 98)
            DrawMod.TGame.EditObj.TempRegimeSlot = selectedItem.ID;
          else if (this.type == 99)
            DrawMod.TGame.EditObj.TempPeopleSlot = selectedItem.ID;
          else if (this.type == 100)
            DrawMod.TGame.EditObj.TempHisModelUnit = selectedItem.ID;
          else if (this.type == 101)
            DrawMod.TGame.EditObj.TempHisUnit = selectedItem.ID;
          else if (this.type == 102)
          {
            if (selectedItem.ID > -1)
              DrawMod.TGame.Data.HistoricalUnitObj[selectedItem.ID].Pool = true;
          }
          else if (this.type == 103)
            DrawMod.TGame.EditObj.TempRandom = selectedItem.ID;
          else if (this.type == 104)
          {
            DrawMod.TGame.Data.StringListObj[this.nr].LibId.libSlot = selectedItem.ID;
            if (DrawMod.TGame.Data.Product == 7)
            {
              if (selectedItem.ID > -1)
              {
                DrawMod.TGame.Data.StringListObj[this.nr].LibId.id =  Math.Round(Conversion.Val(Interaction.InputBox("Give ID for stringlist in this lib")));
                if (DrawMod.TGame.Data.StringListObj[this.nr].LibId.id > this.Data.StringIDCounter)
                  this.Data.StringIDCounter = DrawMod.TGame.Data.StringListObj[this.nr].LibId.id;
              }
              else
                DrawMod.TGame.Data.StringListObj[this.nr].LibId.id = -1;
            }
          }
          else if (this.type == 105)
            this.Data.StringListObj[this.nr].ColValueType[this.nr2] = (NewEnums.LibVarValueType) selectedItem.ID;
          else if (this.type == 106)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 107)
          {
            this.Data.eventPicLibId[this.nr].libSlot = selectedItem.ID;
            if (selectedItem.ID > -1)
            {
              this.Data.eventPicLibId[this.nr].id =  Math.Round(Conversion.Val(Interaction.InputBox("Give ID for eventpic in this lib")));
              if (this.Data.eventPicLibId[this.nr].id > this.Data.EventIdCounter)
                this.Data.EventIdCounter = this.Data.eventPicLibId[this.nr].id;
            }
            else
              this.Data.eventPicLibId[this.nr].id = -1;
          }
          else if (this.type == 108)
          {
            this.Data.SmallLibId[this.nr].libSlot = selectedItem.ID;
            this.Data.SmallLibId[this.nr].id = selectedItem.ID <= -1 ? -1 :  Math.Round(Conversion.Val(Interaction.InputBox("Give ID for eventpic in this lib")));
          }
          else if (this.type == 109)
            this.Data.ActionCardObj[this.nr].LibId.libSlot = selectedItem.ID;
          else if (this.type == 110)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 111)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 112 | this.type >= 113 & this.type <= 117 | this.type ==  sbyte.MaxValue)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type >= 118 & this.type <= 126 | this.type == 128)
            this.Data.LibVarObj[this.nr].value = selectedItem.ID;
          else if (this.type == 129)
            this.Data.ReinfLibId[this.nr].libSlot = selectedItem.ID;
          else if (this.type == 130)
          {
            this.Data.SFTypeObj[this.nr].CopyDataFrom = selectedItem.ID;
            name: String = this.Data.SFTypeObj[this.nr].Name;
            description: String = this.Data.SFTypeObj[this.nr].Description;
            let mut reinforcementType: i32 =  this.Data.SFTypeObj[this.nr].ReinforcementType;
            let mut reinforcementType2: i32 =  this.Data.SFTypeObj[this.nr].ReinforcementType2;
            symbolFileName: String = this.Data.SFTypeObj[this.nr].SymbolFileName;
            sidewaysFileName: String = this.Data.SFTypeObj[this.nr].SidewaysFileName;
            moveWav: String = this.Data.SFTypeObj[this.nr].MoveWAV;
            battleWav: String = this.Data.SFTypeObj[this.nr].BattleWAV;
            let mut id: i32 =  this.Data.SFTypeObj[this.nr].Id;
            int[] numArray1 = new int[100];
            int[] numArray2 = (int[]) this.Data.SFTypeObj[this.nr].SFTypeVar.Clone();
            this.Data.SFTypeObj[this.nr] = this.Data.SFTypeObj[selectedItem.ID].Clone();
            this.Data.SFTypeObj[this.nr].CopyDataFrom = selectedItem.ID;
            this.Data.SFTypeObj[this.nr].Id = id;
            this.Data.SFTypeObj[this.nr].DontShowInList = false;
            if (this.nr2 < 1)
            {
              this.Data.SFTypeObj[this.nr].Name = name;
              this.Data.SFTypeObj[this.nr].Description = description;
              this.Data.SFTypeObj[this.nr].SFTypeVar = (int[]) numArray2.Clone();
              this.Data.SFTypeObj[this.nr].ReinforcementType = reinforcementType;
              this.Data.SFTypeObj[this.nr].ReinforcementType2 = reinforcementType2;
              this.Data.SFTypeObj[this.nr].SymbolFileName = symbolFileName;
              this.Data.SFTypeObj[this.nr].SymbolFileName2 = symbolFileName;
              this.Data.SFTypeObj[this.nr].SidewaysFileName = sidewaysFileName;
              this.Data.SFTypeObj[this.nr].MoveWAV = moveWav;
              this.Data.SFTypeObj[this.nr].MoveWAV = battleWav;
            }
            this.Data.SFTypeObj[this.nr].LoadSprites();
          }
          else if (this.type == 131)
            this.Data.SFObj[this.nr].Type = selectedItem.ID;
          else if (this.type == 132)
            this.Data.SFObj[this.nr].People = selectedItem.ID;
          else if (this.type == 133)
            this.Data.HistoricalUnitObj[this.nr].Type = selectedItem.ID;
          else if (this.type == 134)
            this.Data.HistoricalUnitObj[this.nr].UsePeopleGfx = selectedItem.ID;
          else if (this.type == 135)
            this.Data.HistoricalUnitObj[this.nr].UseModelCounter = selectedItem.ID;
          else if (this.type == 136)
            this.Data.HistoricalUnitObj[this.nr].SubParts[this.nr2] = selectedItem.ID;
          else if (this.type == 137)
            this.Data.HistoricalUnitObj[this.nr].People = selectedItem.ID;
          else if (this.type == 138)
            this.Data.HistoricalUnitObj[this.nr].ModelMaster = selectedItem.ID;
          else if (this.type == 139)
          {
            if (selectedItem.ID > -1)
            {
              if (this.nr2 > this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter)
              {
                this.Game.Data.HistoricalUnitObj[this.nr].DeckCard = (int[]) Utils.CopyArray((Array) this.Game.Data.HistoricalUnitObj[this.nr].DeckCard, (Array) new int[this.nr2 + 1]);
                this.Game.Data.HistoricalUnitObj[this.nr].DeckChance = (int[]) Utils.CopyArray((Array) this.Game.Data.HistoricalUnitObj[this.nr].DeckChance, (Array) new int[this.nr2 + 1]);
                this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter = this.nr2;
              }
            }
            else if (this.nr2 <= this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter)
            {
              this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter = this.nr2 - 1;
              if (this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter == -1)
              {
                this.Game.Data.HistoricalUnitObj[this.nr].DeckCard = new int[1];
                this.Game.Data.HistoricalUnitObj[this.nr].DeckChance = new int[1];
              }
              else
              {
                this.Game.Data.HistoricalUnitObj[this.nr].DeckCard = (int[]) Utils.CopyArray((Array) this.Game.Data.HistoricalUnitObj[this.nr].DeckCard, (Array) new int[this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter + 1]);
                this.Game.Data.HistoricalUnitObj[this.nr].DeckChance = (int[]) Utils.CopyArray((Array) this.Game.Data.HistoricalUnitObj[this.nr].DeckChance, (Array) new int[this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter + 1]);
              }
            }
            if (this.nr2 <= this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter)
              this.Data.HistoricalUnitObj[this.nr].DeckCard[this.nr2] = selectedItem.ID;
            let mut deckCardCounter: i32 =  this.Game.Data.HistoricalUnitObj[this.nr].DeckCardCounter;
            for (let mut index: i32 =  0; index <= deckCardCounter; index += 1)
              this.Game.Data.HistoricalUnitObj[this.nr].DeckChance[index] = 100;
          }
          else if (this.type >= 140 & this.type <= 141)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type >= 142 & this.type <= 143)
            this.Data.LibVarObj[this.nr].value = selectedItem.ID;
          else if (this.type == 144)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 145)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 146)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 147)
          {
            if (selectedItem.ID > -1)
            {
              let mut mapWidth: i32 =  this.Game.Data.MapObj[0].MapWidth;
              for (let mut index7: i32 =  0; index7 <= mapWidth; index7 += 1)
              {
                let mut mapHeight: i32 =  this.Game.Data.MapObj[0].MapHeight;
                for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
                {
                  if (this.Game.Data.MapObj[0].HexObj[index7, index8].LandscapeType == this.nr)
                    this.Game.Data.MapObj[0].HexObj[index7, index8].LandscapeType = selectedItem.ID;
                }
              }
            }
          }
          else if (this.type == 148)
          {
            if (selectedItem.ID > -1)
            {
              let mut mapWidth: i32 =  this.Game.Data.MapObj[0].MapWidth;
              for (let mut index9: i32 =  0; index9 <= mapWidth; index9 += 1)
              {
                let mut mapHeight: i32 =  this.Game.Data.MapObj[0].MapHeight;
                for (let mut index10: i32 =  0; index10 <= mapHeight; index10 += 1)
                {
                  let mut index11: i32 =  0;
                  do
                  {
                    if (this.Game.Data.MapObj[0].HexObj[index9, index10].RoadType[index11] == this.nr)
                      this.Game.Data.MapObj[0].HexObj[index9, index10].RoadType[index11] = selectedItem.ID;
                    index11 += 1;
                  }
                  while (index11 <= 5);
                }
              }
            }
          }
          else if (this.type == 149)
          {
            if (selectedItem.ID > -1)
            {
              let mut mapWidth: i32 =  this.Game.Data.MapObj[0].MapWidth;
              for (let mut index12: i32 =  0; index12 <= mapWidth; index12 += 1)
              {
                let mut mapHeight: i32 =  this.Game.Data.MapObj[0].MapHeight;
                for (let mut index13: i32 =  0; index13 <= mapHeight; index13 += 1)
                {
                  let mut index14: i32 =  0;
                  do
                  {
                    if (this.Game.Data.MapObj[0].HexObj[index12, index13].RiverType[index14] == this.nr)
                      this.Game.Data.MapObj[0].HexObj[index12, index13].RiverType[index14] = selectedItem.ID;
                    index14 += 1;
                  }
                  while (index14 <= 5);
                }
              }
            }
          }
          else if (this.type == 150)
            this.Data.StringListObj[this.nr].Data[this.nr2, this.nr3] = Conversions.ToString(selectedItem.ID);
          else if (this.type == 151)
          {
            if (selectedItem.ID > -1)
            {
              let mut mapWidth: i32 =  this.Game.Data.MapObj[0].MapWidth;
              for (let mut index15: i32 =  0; index15 <= mapWidth; index15 += 1)
              {
                let mut mapHeight: i32 =  this.Game.Data.MapObj[0].MapHeight;
                for (let mut index16: i32 =  0; index16 <= mapHeight; index16 += 1)
                {
                  if (this.Game.Data.MapObj[0].HexObj[index15, index16].Location > -1 && this.Game.Data.LocObj[this.Game.Data.MapObj[0].HexObj[index15, index16].Location].Type == this.nr)
                  {
                    this.Game.Data.LocObj[this.Game.Data.MapObj[0].HexObj[index15, index16].Location].Type = selectedItem.ID;
                    this.Game.Data.LocObj[this.Game.Data.MapObj[0].HexObj[index15, index16].Location].StructuralPts = this.Game.Data.LocTypeObj[selectedItem.ID].StructuralPts;
                  }
                }
              }
            }
          }
          else if (this.type == 153)
          {
            if (selectedItem.ID > -1 && this.Game.HandyFunctionsObj.HexNeighbour(this.Game.SelectX, this.Game.SelectY, 0, selectedItem.ID).onmap)
            {
              this.WindowRef.form3_orderResult = this.Game.ProcessingObj.BlowBridge(this.Game.EditObj.UnitSelected, this.Game.SelectX, this.Game.SelectY, 0, selectedItem.ID - 1);
              this.WindowRef.form3_returnInstruction = true;
              this.WindowRef.form3_data1 = 35;
            }
          }
          else if (this.type == 154 && selectedItem.ID > -1 && this.Game.HandyFunctionsObj.HexNeighbour(this.Game.SelectX, this.Game.SelectY, 0, selectedItem.ID).onmap)
          {
            this.WindowRef.form3_orderResult = this.Game.ProcessingObj.BuildInfra(this.Game.EditObj.UnitSelected, this.Game.SelectX, this.Game.SelectY, 0, selectedItem.ID - 1);
            this.WindowRef.form3_returnInstruction = true;
            this.WindowRef.form3_data1 = 36;
          }
        }
      }
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

     void Button3_Click(object sender, EventArgs e)
    {
      ArrayList arrayList = ArrayList::new();
      let mut num1: i32 =  this.ListBox1.Items.Count - 1;
      for (let mut index: i32 =  0; index <= num1; index += 1)
        arrayList.Add(RuntimeHelpers.GetObjectValue(this.ListBox1.Items[index]));
      SortListArray sortListArray = new SortListArray(true);
      arrayList.Sort((IComparer) sortListArray);
      this.ListBox1.Items.Clear();
      let mut num2: i32 =  arrayList.Count - 1;
      for (let mut index: i32 =  0; index <= num2; index += 1)
        this.ListBox1.Items.Add(RuntimeHelpers.GetObjectValue(arrayList[index]));
    }
  }
}
